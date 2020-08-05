# Copyright (c) 2020  Teddy Wing
#
# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.
#
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
# GNU General Public License for more details.
#
# You should have received a copy of the GNU General Public License
# along with this program. If not, see <https://www.gnu.org/licenses/>.


SOURCES := $(shell find . -name '*.rs')
MAN_PAGES := $(patsubst doc/%.1.txt,doc/%.1,$(wildcard doc/*.1.txt))

# PRODUCTS := $(patsubst src/bin/%.rs,target/release/%,$(wildcard src/bin/*.rs))
PRODUCTS := $(patsubst src/bin/%.rs,%,$(wildcard src/bin/*.rs))
RELEASE_PRODUCTS := $(patsubst %,target/release/%,$(PRODUCTS))

DIST := $(abspath dist)
DIST_PRODUCTS := $(patsubst %,dist/%,$(PRODUCTS))
DIST_MAN_PAGES := $(patsubst doc/%,dist/%,$(MAN_PAGES))


.PHONY: doc
doc: $(MAN_PAGES)

doc/%.1: doc/%.1.txt
	sed 's/`/*/g' $< > $@.transformed
	a2x --no-xmllint --format manpage $@.transformed
	rm $@.transformed


$(RELEASE_PRODUCTS): $(SOURCES)
	cargo build --release


.PHONY: dist
dist: $(DIST_PRODUCTS) $(DIST_MAN_PAGES)

$(DIST):
	mkdir -p $@

$(DIST_PRODUCTS): $(DIST) $(RELEASE_PRODUCTS)
	cp $(RELEASE_PRODUCTS) $(DIST)

$(DIST_MAN_PAGES): $(DIST) $(MAN_PAGES)
	cp $(MAN_PAGES) $(DIST)
