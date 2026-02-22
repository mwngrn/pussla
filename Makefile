.PHONY: validate aggregate test all help

all: validate aggregate test

help:
	@echo "Pussla - Planning as Code"
	@echo ""
	@echo "Usage:"
	@echo "  make validate   Run data validation (schema, PII, cross-refs)"
	@echo "  make aggregate  Generate weekly allocation summary"
	@echo "  make test       Run unit tests"
	@echo "  make all        Run everything"

validate:
	python3 src/validate_planning_data.py

aggregate:
	python3 src/aggregate_planning_data.py

test:
	python3 tests/test_validation.py
	python3 tests/test_aggregation.py
