MAN_PAGES := $(patsubst doc/%.1.txt,doc/%.1,$(wildcard doc/*.1.txt))

.PHONY: doc
doc: $(MAN_PAGES)

doc/%.1: doc/%.1.txt
	sed 's/`/*/g' $< > $@.transformed
	a2x --no-xmllint --format manpage $@.transformed
	rm $@.transformed
