.PHONY: clean test coverage build install lint

# ============================================================================ #
# CLEAN COMMANDS
# ============================================================================ #

clean: clean-build clean-pyc clean-test clean-stubs  ## remove all build, test, coverage and Python artifacts

clean-build: ## remove build artifacts
	rm -rf dist/
	rm -rf build/
	rm -rf target/

clean-stubs:
	find . -name '*.pyi' -exec rm -f {} +


clean-pyc: ## remove Python file artifacts
	find . -name '*.pyc' -exec rm -f {} +
	find . -name '*.pyo' -exec rm -f {} +
	find . -name '*~' -exec rm -f {} +
	find . -name '__pycache__' -exec rm -fr {} +

clean-test: ## remove test artifacts
	rm -rf .pytest_cache/
	rm -rf .coverage
	rm -rf .coverage.*
	rm -rf htmlcov/
	rm -rf .mypy_cache/
	rm -rf .ruff_cache/
	rm -rf .pytest_cache/

# ============================================================================ #
# UPDATE COMMANDS
# ============================================================================ #

update:
	cargo update --verbose

# ============================================================================ #
# INSTALL COMMANDS
# ============================================================================ #

install:
	uv sync --dev --all-extras

develop: ## install the package to the active Python's site-packages
	maturin develop --uv --release

build: clean update stubs ## install the package to the active Python's site-packages
	maturin build --release

stubs: ## generate *.pyi stubs file
	cargo run --no-default-features --features generate-stubs --bin stub_gen
