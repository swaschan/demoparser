import os
from pathlib import Path
import subprocess
import tempfile

REPO_URL = "https://github.com/SteamDatabase/GameTracking-CS2.git"
REPO_REVISION = "61e6f2a613357c7e70de957cf41e06ef840ff6ba"
CRATE_DIR = Path(__file__).resolve().parent


with tempfile.TemporaryDirectory() as temporary_directory:
    checkout = Path(temporary_directory) / "GameTracking-CS2"
    subprocess.run(["git", "init", str(checkout)], check=True)
    subprocess.run(
        ["git", "-C", str(checkout), "remote", "add", "origin", REPO_URL],
        check=True,
    )
    subprocess.run(
        [
            "git",
            "-C",
            str(checkout),
            "fetch",
            "--depth=1",
            "origin",
            REPO_REVISION,
        ],
        check=True,
    )
    subprocess.run(
        ["git", "-C", str(checkout), "checkout", "--detach", "FETCH_HEAD"],
        check=True,
    )

    environment = os.environ.copy()
    environment["CSGOPROTO_REGENERATE"] = "1"
    environment["CSGOPROTO_PROTO_DIR"] = str(checkout / "Protobufs")
    subprocess.run(
        ["cargo", "build", "--manifest-path", str(CRATE_DIR / "Cargo.toml")],
        check=True,
        env=environment,
    )
