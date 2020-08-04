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


MAN_PAGES := $(patsubst doc/%.1.txt,doc/%.1,$(wildcard doc/*.1.txt))

.PHONY: doc
doc: $(MAN_PAGES)

doc/%.1: doc/%.1.txt
	sed 's/`/*/g' $< > $@.transformed
	a2x --no-xmllint --format manpage $@.transformed
	rm $@.transformed
