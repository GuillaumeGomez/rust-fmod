# Rust-FMOD - Copyright (c) 2014 Gomez Guillaume.
#
# The Original software, FMOD library, is provided by FIRELIGHT TECHNOLOGIES.
#
# This software is provided 'as-is', without any express or implied warranty.
# In no event will the authors be held liable for any damages arising from
# the use of this software.
#
# Permission is granted to anyone to use this software for any purpose,
# including commercial applications, and to alter it and redistribute it
# freely, subject to the following restrictions:
#
# 1. The origin of this software must not be misrepresented; you must not claim
#    that you wrote the original software. If you use this software in a product,
#    an acknowledgment in the product documentation would be appreciated but is
#    not required.
#
# 2. Altered source versions must be plainly marked as such, and must not be
#    misrepresented as being the original software.
#
# 3. This notice may not be removed or altered from any source distribution.

all: rfmod examples doc

rfmod:
	mkdir -p lib
	rustc --out-dir=lib src/rfmod.rs

examples: rfmod
	  mkdir -p bin
	  rustc -o bin/simple_music_player -L ./lib examples/simple_music_player/src/simple_music_player.rs
	  rustc -o bin/recording -L ./lib examples/recording/src/recording.rs
	  rustc -o bin/dsp_custom -L ./lib examples/dsp_custom/src/dsp_custom.rs
	  rustc -o bin/user_created_sound -L ./lib examples/user_created_sound/src/user_created_sound.rs
	  rustc -o bin/3d -L ./lib examples/3d/src/3d.rs
	  rustc -o bin/effects -L ./lib examples/effects/src/effects.rs
	  rustc -o bin/file_callbacks -L ./lib examples/file_callbacks/src/file_callbacks.rs

doc:
	rustdoc -o doc src/rfmod.rs

clean:
	rm -rf lib
	rm -rf bin/simple_music_player
	rm -rf bin/recording
	rm -rf bin/dsp_custom
	rm -rf bin/user_created_sound
	rm -rf bin/3d
	rm -rf bin/effects
	rm -rf bin/file_callbacks

re: clean all
