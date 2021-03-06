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


VERSION := $(shell fgrep 'const VERSION' src/lib.rs | awk -F '"' '{ print $$2 }')
TOOLCHAIN := $(shell fgrep default_host_triple $(HOME)/.rustup/settings.toml | awk -F '"' '{ print $$2 }')

SOURCES := $(shell find . -name '*.rs')
MAN_PAGES := $(patsubst doc/%.1.txt,doc/%.1,$(wildcard doc/*.1.txt))

PRODUCTS := $(patsubst src/bin/%.rs,%,$(wildcard src/bin/*.rs))
RELEASE_PRODUCTS := $(patsubst %,target/release/%,$(PRODUCTS))

DIST := $(abspath dist)
DIST_PRODUCTS := $(patsubst %,dist/%,$(PRODUCTS))
DIST_MAN_PAGES := $(patsubst doc/%,dist/%,$(MAN_PAGES))

# Set STATIC=1 to build a static binary.
STATIC ?= 0

ifeq ($(STATIC), 1)
BUILD_VARS += PKG_CONFIG_LIBDIR=''
endif


.PHONY: doc
doc: $(MAN_PAGES)

doc/%.1: doc/%.1.txt
	sed 's/`/*/g' $< > $@.transformed
	a2x --no-xmllint --format manpage $@.transformed
	rm $@.transformed


$(RELEASE_PRODUCTS): $(SOURCES)
	$(BUILD_VARS) cargo build --release


.PHONY: dist
dist: $(DIST_PRODUCTS) $(DIST_MAN_PAGES)

$(DIST):
	mkdir -p $@

$(DIST)/bin: $(DIST)
	mkdir -p $@

$(DIST)/share/man/man1: $(DIST)
	mkdir -p $@

$(DIST_PRODUCTS): $(DIST)/bin $(RELEASE_PRODUCTS)
	cp $(RELEASE_PRODUCTS) $<

$(DIST_MAN_PAGES): $(DIST)/share/man/man1 $(MAN_PAGES)
	cp $(MAN_PAGES) $<


.PHONY: pkg
pkg: git-suggestion_$(VERSION)_$(TOOLCHAIN).tar.bz2

git-suggestion_$(VERSION)_$(TOOLCHAIN).tar.bz2: dist
	tar cjv -s /dist/git-suggestion_$(VERSION)_$(TOOLCHAIN)/ -f $@ dist
