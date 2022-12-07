# No (file) targets are assumed for most Makefile commands.
.PHONY:
	init_day

help: ## Print help.
	@echo Possible commands:
	@cat Makefile | grep '##' | grep -v "Makefile" | sed -e 's/^/  - /'

init_day: ## Initialize a new AoC day.
	@cargo new "$(YEAR)"/day"$(DAY)" -q
	@cp -r template/data "$(YEAR)"/day"$(DAY)"/
	@cp -f template/src/main.rs "$(YEAR)"/day"$(DAY)"/src/main.rs
	@cp -r template/.vscode "$(YEAR)"/day"$(DAY)"/.vscode
	@sed -i.bak 's/dayXX/day$(DAY)/g' "$(YEAR)"/day"$(DAY)"/.vscode/launch.json
	@rm "$(YEAR)"/day"$(DAY)"/.vscode/launch.json.bak
	@echo "$(YEAR)"/day"$(DAY)" initialized!
