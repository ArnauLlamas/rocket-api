HELP_FUN = \
		 %help; \
		 while(<>) { \
		 	push @{$$help{$$2 // 'options'}}, [$$1, $$3] if /^([a-zA-Z0-9_-]+)\s*:.*\#\#(?:@(\w+))?\s(.*)$$/ \
		 }; \
		 print "usage: make [target]\nexample: make help\n\n"; \
		 for ( sort keys %help ) { \
		 	print "$$_:\n"; \
			printf("  %-20s %s\n", $$_->[0], $$_->[1]) for @{$$help{$$_}}; \
			print "\n"; \
		 }

help: ##@Miscellaneous Show this help
	@perl -e '$(HELP_FUN)' $(MAKEFILE_LIST)

## TODO
