VENV_NAME?=.venv

USER_PYTHON ?= python3
VENV_PYTHON=${VENV_NAME}/bin/python

.PHONY = build clean doc format install-python lint prepare-venv


$(VENV_NAME)/bin/python:
	make clean && ${USER_PYTHON} -m venv $(VENV_NAME)

build:
	cargo build

clean:
	rm -rf $(VENV_NAME)
	rm -rf .pytest_cache
	rm -rf .ruff_cache
	rm -rf pyo3_graphster.egg-info
	rm -rf target
	rm -rf dist
	rm -rf pyo3_graphster/*.so
	find . -name __pycache__ -type d -exec rm -r {} +

doc:
	RUSTDOCFLAGS="--cfg docsrs" cargo +nightly doc --no-deps --all-features

format: install-python
	cargo fmt
	${VENV_PYTHON} -m ruff check --select I --fix
	${VENV_PYTHON} -m ruff format

install-python: prepare-venv
	${VENV_PYTHON} -m pip install --upgrade pip
	${VENV_PYTHON} -m pip install -e .\[dev\]

lint:
	cargo clippy --all-targets --all-features -- -D warnings
	${VENV_PYTHON} -m ruff check
	${VENV_PYTHON} -m ruff check --select I
	${VENV_PYTHON} -m pyright

prepare-venv: $(VENV_NAME)/bin/python
