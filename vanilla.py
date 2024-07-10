#!/usr/bin/env python3

import argparse
import asyncio
import asyncio.subprocess
from datetime import datetime
import json
import subprocess
import sys


async def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--getter-count", type=int, required=True)
    parser.add_argument("--getter-interval", type=int, required=True)
    parser.add_argument("--queryable-interval", type=int, required=True)
    args = parser.parse_args()

    subprocess.run(
        [
            "cargo",
            "build",
            "--release",
            "--bin",
            "vanilla_slow_queryable",
        ]
    )

    queryable = await asyncio.subprocess.create_subprocess_exec(
        "target/release/vanilla_slow_queryable",
        "vanilla.config.json5",
        str(args.queryable_interval),
        stdout=asyncio.subprocess.PIPE,
        stderr=asyncio.subprocess.PIPE,
    )

    subprocess.run(
        [
            "cargo",
            "build",
            "--release",
            "--bin",
            "vanilla_repeated_get",
        ]
    )

    getters = [
        subprocess.Popen(
            [
                "target/release/vanilla_repeated_get",
                "peer.vanilla.config.json5",
                str(args.getter_interval),
            ],
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
        )
        for _ in range(args.getter_count)
    ]
    print(f"Spawned {len(getters)} getters")

    interval_ms = None
    preivous_timestamp = None
    read_tolerance_ratio = 10
    test_tolerance_ratio = 3
    success = True

    while True:
        try:
            try:
                data = await asyncio.wait_for(
                    read_rx_heartbeat(queryable),
                    (
                        interval_ms * read_tolerance_ratio / 1_000
                        if interval_ms is not None
                        else None
                    ),  # seconds
                )
            except asyncio.TimeoutError:
                success = False
                print(
                    f"Didn't get RX heartbeat after {interval_ms * read_tolerance_ratio}ms"
                )
                break

            if interval_ms is None:
                interval_ms = float(data["fields"]["interval_ms"])
                continue

            timestamp = datetime.strptime(
                data["timestamp"],
                "%Y-%m-%dT%H:%M:%S.%fZ",
            )

            if preivous_timestamp is None:
                preivous_timestamp = timestamp
                continue

            dt = timestamp - preivous_timestamp
            dt_ms = dt.microseconds / 1_000
            if dt_ms > interval_ms * test_tolerance_ratio:
                success = False
                break

            print(f"Got RX hearbeat in {dt_ms}ms")

            preivous_timestamp = timestamp
        except asyncio.CancelledError:
            queryable.kill()
            for getter in getters:
                getter.kill()
            print("Vanilla test was cancelled")
            return

    queryable.kill()
    for getter in getters:
        getter.kill()

    if success:
        print("Vanilla test succeeded!")
    else:
        print("Vanilla test failed!")
        sys.exit(1)


async def read_rx_heartbeat(proc):
    while True:
        line = await proc.stdout.readline()
        data = json.loads(line)

        if (
            data["target"] == "zenoh::instrumentation::heartbeat::runtime"
            and data["fields"]["runtime"] == "rx"
        ):
            return data


if __name__ == "__main__":
    asyncio.run(main())
