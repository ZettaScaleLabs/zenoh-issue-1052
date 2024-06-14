import subprocess


def make_stdout_file(i):
    return open(f"vanilla/{i}.stdout", "x")


def make_stderr_file(i):
    return open(f"vanilla/{i}.stderr", "x")


def spawn_procs(n):
    procs = {}

    for i in range(n):
        procs[i] = subprocess.Popen(
            ["target/release/fast_get", "peer.vanilla.config.json5"],
            stdout=make_stdout_file(i),
            stderr=make_stderr_file(i),
        )

    return procs


def kill_procs(procs):
    for proc in procs.values():
        proc.kill()
