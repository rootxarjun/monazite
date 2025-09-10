#!/bin/bash -Cue

export CARGO_NET_GIT_FETCH_WITH_CLI=true

# depName=arkedge/gaia datasource=github-releases
TMTC_C2A_VERSION="1.2.0"
cargo binstall --root . tmtc-c2a --git https://github.com/arkedge/gaia.git --version ${TMTC_C2A_VERSION} --no-confirm --force

# depName=tlmcmddb-cli datasource=crate
TLMCMDDB_CLI_VERSION="2.6.1"
cargo binstall --root . tlmcmddb-cli    --version ${TLMCMDDB_CLI_VERSION} --no-confirm --force

# depName=kble datasource=crate
KBLE_VERSION="0.4.2"
cargo binstall --root . kble            --version ${KBLE_VERSION} --no-confirm --force

# depName=kble-c2a datasource=crate
KBLE_C2A_VERSION="0.4.2"
cargo binstall --root . kble-c2a        --version ${KBLE_C2A_VERSION} --no-confirm --force

# depName=kble-eb90 datasource=crate
KBLE_EB90_VERSION="0.4.2"
cargo binstall --root . kble-eb90       --version ${KBLE_EB90_VERSION} --no-confirm --force

# please release with semver......
curl -o ./bin/jrsonnet -L https://github.com/CertainLach/jrsonnet/releases/download/v0.5.0-pre96-test/jrsonnet-linux-amd64
