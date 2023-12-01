# No (file) targets are assumed for most Makefile commands.


## help: Print help.
.PHONY: help
help:
	@echo Possible commands:
	@cat Makefile | grep '##' | grep -v "Makefile" | sed -e 's/^/  - /'

## initialize_day: Initialize a new AoC day.
.PHONY: init_day
initialize_day:
	@mkdir -p "$(YEAR)"
	@cargo new "$(YEAR)"/day"$(DAY)" -q
	@cp -r template/data "$(YEAR)"/day"$(DAY)"/
	@cp -f template/src/main.rs "$(YEAR)"/day"$(DAY)"/src/main.rs
	@cp -r template/.vscode "$(YEAR)"/day"$(DAY)"/.vscode
	@sed -i.bak 's/dayXX/day$(DAY)/g' "$(YEAR)"/day"$(DAY)"/.vscode/launch.json
	@rm "$(YEAR)"/day"$(DAY)"/.vscode/launch.json.bak
	@echo "$(YEAR)"/day"$(DAY)" initialized!
