{
        description = "parser";

        inputs = {
                nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.11";
                rust-overlay.url = "github:oxalica/rust-overlay";
        };

        outputs = { self, nixpkgs, rust-overlay, ... } @ inputs: let
                t = "\t";
                script = {
                        flake-update = {
                                name = "flake-update";
                                code = ''
                                        LATEST_NIXOS_VERSION=""
                                        function get_latest_nixos_version() {
                                        ${t}printf 'Retrieving latest NixOS version .. '
                                        ${t}LATEST_NIXOS_VERSION=''$(
                                        ${t}${t}wget -O - -q https://nixos.org/download/ |
                                        ${t}${t}${t}grep latest-nixos-minimal-x86_64-linux.iso |
                                        ${t}${t}${t}sed -e 's#^.*"https://channels.nixos.org/nixos-\([^/]\+\)/latest-nixos-minimal-x86_64-linux.iso".*''$#\1#'
                                        ${t})
                                        ${t}printf '%s\n' ''${LATEST_NIXOS_VERSION}
                                        }

                                        OUR_NIXOS_VERSION=""
                                        GITHUB='github:NixOS/nixpkgs/'
                                        function get_our_nixos_version() {
                                        ${t}printf 'Getting our NixOS version       .. '
                                        ${t}OUR_NIXOS_VERSION=''$(
                                        ${t}${t}grep '"'$GITHUB'nixos-[^"]*"' flake.nix |
                                        ${t}${t}${t}sed 's#^[^"]*"'$GITHUB'nixos-\([^"]*\)";''$#\1#'
                                        ${t})
                                        ${t}printf '%s\n' ''${OUR_NIXOS_VERSION}
                                        }

                                        function main() {
                                        ${t}get_latest_nixos_version
                                        ${t}get_our_nixos_version
                                        ${t}if [ "''${OUR_NIXOS_VERSION}" == "''${LATEST_NIXOS_VERSION}" ]; then
                                        ${t}${t}echo 'NixOS version is up-to-date'
                                        ${t}else
                                        ${t}${t}sed -i -e "s/''${OUR_NIXOS_VERSION}/''${LATEST_NIXOS_VERSION}/" flake.nix
                                        ${t}${t}echo 'Updated NixOS version in flake.nix'
                                        ${t}fi
                                        ${t}nix flake update
                                        }

                                        main "''$@"
                                '';
                        };
                        make = {
                                name = "make";
                                code = ''
                                        function log() {
                                        ${t}local level="$1"
                                        ${t}local message="$2"
                                        ${t}printf '%23s | %-5s -- %s\n' "$(date '+%Y-%m-%d %H:%M:%S.%3N')" "$level" "$message"
                                        }

                                        function info() {
                                        ${t}local message="$1"
                                        ${t}log 'INFO' "$message"
                                        }

                                        function warn() {
                                        ${t}local message="$1"
                                        ${t}log 'WARN' "$message"
                                        }

                                        function error() {
                                        ${t}local message="$1"
                                        ${t}log 'ERROR' "$message"
                                        }

                                        function fatal() {
                                        ${t}local message="$1"
                                        ${t}log 'FATAL' "$message"
                                        }

                                        function die() {
                                        ${t}local message="$1"
                                        ${t}fatal "$message"
                                        ${t}exit -1
                                        }

                                        function usage() {
                                        ${t}local script_name=$(basename $0)
                                        ${t}cat <<-EOF
                                        ${t}${t}Usage: $script_name [--help|-h]

                                        ${t}${t}Options:
                                        ${t}${t}    --help|-h
                                        ${t}${t}        Shows this message

                                        ${t}${t}    clean
                                        ${t}${t}        Run "cargo clean"

                                        ${t}${t}    format
                                        ${t}${t}        Run "cargo fmt"

                                        ${t}${t}    check
                                        ${t}${t}        Run "cargo check"

                                        ${t}${t}    lint
                                        ${t}${t}        Run "cargo clippy"

                                        ${t}${t}    test
                                        ${t}${t}        Run "cargo test"

                                        ${t}${t}    --coverage|-c
                                        ${t}${t}        Run "cargo llvm-cov --html" instead of "cargo test"

                                        ${t}${t}    --no-check
                                        ${t}${t}        Do not check the coverage percentages

                                        ${t}${t}    build
                                        ${t}${t}        Run "cargo build"

                                        ${t}${t}    all
                                        ${t}${t}        Run all ("format", "check", "lint", "test", and "build")

                                        ${t}${t}    println
                                        ${t}${t}        Find all occurrences of "println!(...)"

                                        ${t}${t}    --ignore-main
                                        ${t}${t}        Ignore "main.rs" when looking for occurrences of "println!(...)"
                                        ${t}EOF
                                        ${t}exit 0
                                        }

                                        CLEAN='F'
                                        function clean {
                                        ${t}info "Clean"
                                        ${t}cargo clean || die 'Cleaning failed'
                                        ${t}info "Clean done"
                                        }

                                        FORMAT='F'
                                        function format {
                                        ${t}info "Format"
                                        ${t}cargo-fmt || die 'Formatting failed'
                                        ${t}info "Format done"
                                        }

                                        CHECK='F'
                                        function check {
                                        ${t}info "Check"
                                        ${t}RUSTFLAGS=-Awarnings cargo check || die 'Checking failed'
                                        ${t}info "Check done"
                                        }

                                        LINT='F'
                                        function lint {
                                        ${t}info "Lint"
                                        ${t}cargo clippy || die 'Linting failed'
                                        ${t}info "Lint done"
                                        }

                                        FIND_PRINTLN='F'
                                        IGNORE_MAIN='F'
                                        function find_println {
                                        ${t}info "Find 'println!(...)'"
                                        ${t}if [ $IGNORE_MAIN == 'T' ]; then
                                        ${t}${t}find . -type f -path './src/main.rs' -prune -o -type f -name '*.rs' -print | xargs grep -n 'println!'
                                        ${t}else
                                        ${t}${t}find . -type f -name '*.rs' -print | xargs grep -n 'println!'
                                        ${t}fi
                                        ${t}info "Find 'println!(...)' done"
                                        }

                                        TEST='F'
                                        COVERAGE='F'
                                        CHECK_PERCENTAGES='T'
                                        MIN_FUNCTION_COVERAGE_PERCENTAGE=0
                                        MIN_LINE_COVERAGE_PERCENTAGE=0
                                        MIN_REGION_COVERAGE_PERCENTAGE=0
                                        function init {
                                        ${t}if [ -f .percentages ]; then
                                        ${t}${t}local percentages=''$(echo ''$(cat .percentages))
                                        ${t}${t}MIN_FUNCTION_COVERAGE_PERCENTAGE=''$(echo ''${percentages%%|*})
                                        ${t}${t}percentages=''${percentages#*|}
                                        ${t}${t}MIN_LINE_COVERAGE_PERCENTAGE=''$(echo ''${percentages%%|*})
                                        ${t}${t}percentages=''${percentages#*|}
                                        ${t}${t}MIN_REGION_COVERAGE_PERCENTAGE=''$(echo ''${percentages%%|*})
                                        ${t}fi
                                        }

                                        DIRTY='F'
                                        INDICATION=""
                                        function compare_percentages {
                                        ${t}local actual=''$1
                                        ${t}local expected=''$2
                                        ${t}local name=''$3
                                        ${t}if (( ''$(echo "''$actual < ''$expected" | bc -l) )); then
                                        ${t}${t}if [ $CHECK_PERCENTAGES == 'T' ]; then
                                        ${t}${t}${t}echo "Not enough ''$name coverage (''$actual < ''$expected)!"
                                        ${t}${t}${t}exit -1
                                        ${t}${t}else
                                        ${t}${t}${t}INDICATION='(↓↓↓)'
                                        ${t}${t}fi
                                        ${t}elif (( ''$(echo "''$actual > ''$expected" | bc -l) )); then
                                        ${t}${t}DIRTY='T'
                                        ${t}${t}INDICATION='(↑↑↑)'
                                        ${t}else
                                        ${t}${t}INDICATION=""
                                        ${t}fi
                                        }

                                        COVERAGE_RESULT=' '
                                        function check_coverage {
                                        ${t}local result=$COVERAGE_RESULT
                                        ${t}result="''${result#Totals }"
                                        ${t}local function_coverage="''$(echo ''${result%%% (*})"
                                        ${t}compare_percentages "''$function_coverage" "''$MIN_FUNCTION_COVERAGE_PERCENTAGE" 'function'
                                        ${t}local function_indication="''$INDICATION"
                                        ${t}result=''${result#*) }
                                        ${t}local line_coverage="''$(echo ''${result%%% (*})"
                                        ${t}compare_percentages "''$line_coverage" "''$MIN_LINE_COVERAGE_PERCENTAGE" 'line'
                                        ${t}local line_indication="''$INDICATION"
                                        ${t}result=''${result#*) }
                                        ${t}local region_coverage="''$(echo ''${result%%% (*})"
                                        ${t}compare_percentages "''$region_coverage" "''$MIN_REGION_COVERAGE_PERCENTAGE" 'region'
                                        ${t}local region_indication="''$INDICATION"
                                        ${t}if [ ''$DIRTY == 'T' ]; then
                                        ${t}${t}echo "''$function_coverage|''$line_coverage|''$region_coverage" >.percentages
                                        ${t}fi
                                        ${t}echo ""
                                        ${t}echo "    Coverage:"
                                        ${t}echo "        Function: ''$function_coverage''${function_indication}; Line: ''$line_coverage''${line_indication}; Region: ''$region_coverage''${region_indication}"
                                        ${t}echo ""
                                        }

                                        function test {
                                        ${t}info "Test"
                                        ${t}if [ $COVERAGE == 'F' ]; then
                                        ${t}${t}cargo test || die 'Testing failed'
                                        ${t}else
                                        ${t}${t}cargo llvm-cov --html || die 'Testing failed'
                                        ${t}${t}init
                                        ${t}${t}COVERAGE_RESULT=''$(xidel target/llvm-cov/html/index.html -e '//tr[last()]')
                                        ${t}${t}check_coverage
                                        ${t}fi
                                        ${t}info "Test done"
                                        }

                                        BUILD='F'
                                        function build {
                                        ${t}info "Build"
                                        ${t}cargo build || die 'Building failed'
                                        ${t}info "Build done"
                                        }

                                        function main() {
                                        ${t}while [ $# -gt 0 ]; do
                                        ${t}${t}case $1 in
                                        ${t}${t}${t}--help|-h)
                                        ${t}${t}${t}${t}usage
                                        ${t}${t}${t}${t};;

                                        ${t}${t}${t}clean)
                                        ${t}${t}${t}${t}CLEAN='T'
                                        ${t}${t}${t}${t};;

                                        ${t}${t}${t}format)
                                        ${t}${t}${t}${t}FORMAT='T'
                                        ${t}${t}${t}${t};;

                                        ${t}${t}${t}check)
                                        ${t}${t}${t}${t}CHECK='T'
                                        ${t}${t}${t}${t};;

                                        ${t}${t}${t}lint)
                                        ${t}${t}${t}${t}LINT='T'
                                        ${t}${t}${t}${t};;

                                        ${t}${t}${t}test)
                                        ${t}${t}${t}${t}TEST='T'
                                        ${t}${t}${t}${t};;

                                        ${t}${t}${t}--coverage|-c)
                                        ${t}${t}${t}${t}COVERAGE='T'
                                        ${t}${t}${t}${t};;

                                        ${t}${t}${t}--no-check)
                                        ${t}${t}${t}${t}CHECK_PERCENTAGES='F'
                                        ${t}${t}${t}${t};;

                                        ${t}${t}${t}build)
                                        ${t}${t}${t}${t}BUILD='T'
                                        ${t}${t}${t}${t};;

                                        ${t}${t}${t}all)
                                        ${t}${t}${t}${t}FORMAT='T'
                                        ${t}${t}${t}${t}CHECK='T'
                                        ${t}${t}${t}${t}LINT='T'
                                        ${t}${t}${t}${t}TEST='T'
                                        ${t}${t}${t}${t}BUILD='T'
                                        ${t}${t}${t}${t};;

                                        ${t}${t}${t}println)
                                        ${t}${t}${t}${t}FIND_PRINTLN='T'
                                        ${t}${t}${t}${t};;

                                        ${t}${t}${t}--ignore-main)
                                        ${t}${t}${t}${t}IGNORE_MAIN='T'
                                        ${t}${t}${t}${t};;

                                        ${t}${t}${t}*)
                                        ${t}${t}${t}${t}die "Unknown option: '$1'; aborting..."
                                        ${t}${t}${t}${t};;
                                        ${t}${t}esac
                                        ${t}${t}shift
                                        ${t}done
                                        ${t}if [ $CLEAN == 'T' ]; then
                                        ${t}${t}clean
                                        ${t}fi
                                        ${t}if [ $FORMAT == 'T' ]; then
                                        ${t}${t}format
                                        ${t}fi
                                        ${t}if [ $CHECK == 'T' ]; then
                                        ${t}${t}check
                                        ${t}fi
                                        ${t}if [ $LINT == 'T' ]; then
                                        ${t}${t}lint
                                        ${t}fi
                                        ${t}if [ $TEST == 'T' ]; then
                                        ${t}${t}test
                                        ${t}fi
                                        ${t}if [ $BUILD == 'T' ]; then
                                        ${t}${t}build
                                        ${t}fi
                                        ${t}if [ $FIND_PRINTLN == 'T' ]; then
                                        ${t}${t}find_println
                                        ${t}fi
                                        }

                                        main "$@"
                                '';
                        };
                };
                forAllSystems = flake: nixpkgs.lib.genAttrs nixpkgs.lib.systems.flakeExposed (system: flake rec {
                        pkgs = import nixpkgs {
                                inherit system;
                                overlays = [ (import rust-overlay) ];
                        };
                        script-bin = {
                                flake-update = pkgs.writeShellScriptBin
                                        script.flake-update.name
                                        script.flake-update.code;
                                make = pkgs.writeShellScriptBin
                                        script.make.name
                                        script.make.code;
                        };
                });
        in {
                devShells = forAllSystems ({ pkgs, script-bin, ... }: rec {
                        default = with pkgs; mkShellNoCC {
                                RUST_SRC_PATH = "${rust-bin.nightly.latest.default.override {
                                        extensions = [ "rust-src" ];
                                }}/lib/rustlib/src/rust/library";
                                packages = [
                                        (rust-bin.selectLatestNightlyWith (toolchain: toolchain.default))
                                        bashInteractive
                                        bc
                                        cargo-expand
                                        cargo-llvm-cov
                                        clippy
                                        git
                                        grcov
                                        llvmPackages.llvm
                                        rust-analyzer
                                        xidel

                                        script-bin.flake-update
                                        script-bin.make
                                ];
                                env = {
                                        inherit (pkgs.cargo-llvm-cov) LLVM_COV LLVM_PROFDATA;
                                };
                        };
                });
        };
}
