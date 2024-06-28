import subprocess
import json
from typing import Dict
import os
import shutil


def make_config(i):
    return {
        "mode": "peer",
        "connect": {
            "endpoints": ["tcp/localhost:7447"],
        },
        "listen": {
            "endpoints": [f"tcp/localhost:{9000 + i}"],
        },
        "scouting": {
            "multicast": {
                "enabled": False,
            },
            "gossip": {
                "enabled": False,
            },
        },
        "plugins": {
            "storage_manager": {
                "__path__": [
                    "~/Source/zenoh/target/release/libzenoh_plugin_storage_manager.dylib",
                ],
                "volumes": {
                    "fs": {
                        "__path__": [
                            "~/Source/zenoh-backend-filesystem/target/release/libzenoh_backend_fs.dylib",
                        ],
                    },
                },
                "storages": {
                    "issue_1052": {
                        "key_expr": "zenoh/issues/1052/*",
                        "volume": {
                            "id": "fs",
                            "dir": f"issue-1052-{i}",
                        },
                        "replica_config": {
                            "publication_interval": 20,
                            "propagation_delay": 200,
                            "delta": 1000,
                        },
                    },
                },
            },
        },
    }


def make_config_path(i):
    return f"replication/{i}.peer.replication.config.json5"


def make_stdout_file(i):
    return open(f"replication/{i}.peer.replication.stdout", "x")


def make_stderr_file(i):
    return open(f"replication/{i}.peer.replication.stderr", "x")


class Experiment:
    procs: Dict[int, subprocess.Popen]

    def __init__(self, n):
        self.procs = {}

        if not os.path.exists("replication"):
            os.mkdir("replication")

        for i in range(n):
            config = make_config(i)
            path = make_config_path(i)
            with open(path, "x") as c:
                c.write(json.dumps(config))
            self.procs[i] = subprocess.Popen(
                [
                    "../zenoh/target/release/zenohd",
                    "--config",
                    path,
                    "--id",
                    f"a{i}",
                ],
                stdout=make_stdout_file(i),
                stderr=make_stderr_file(i),
            )

    def cleanup(self):
        for proc in self.procs.values():
            proc.kill()
        shutil.rmtree("replication")
