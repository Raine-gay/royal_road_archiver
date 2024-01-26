### Terminal based Archival Program for RoyalRoad.
#### Currently supports:
* Epub generation - Turn the webnovels into an epub file for offline reading / archival.
* Markdown generation - Turn the webnovels into a markdown file. ~~Dunno why you'd want to do this but hey, you can.~~

#### Coming soon:
* Audiobook generation support.
* HTML + CSS archival. Think archive.org, but on your machine.

### How to use:
* Windows - Download the binaries (.exe files) for windows from the [releases page,](https://github.com/Raine-gay/royal_road_archiver/releases/tag/RoyalRoad_Archiver-Release)    
  Unzip them into a folder E.G Downloads,  
  Open Command prompt,  
  Change the directory to where you unzipped the binaries (.exe files) by typing ``cd <Binaries_Directory>\royal_road_archiver_windows-bin`` E.G ``cd Downloads\royal_road_archiver_windows-bin``,
  Open the program by typing ``royal_road_archiver.exe``

* Linux - Basically the same as windows:
  Download the linux binaries from the [releases page,](https://github.com/Raine-gay/royal_road_archiver/releases/tag/RoyalRoad_Archiver-Release)
  Unzip em,
  Open console,
  ``cd`` to the folder containing the binaries,
  Run it using ``royal_road_archiver``

### Example commands:
* ``royal_road_archiver https://www.royalroad.com/fiction/59450/bioshifter epub`` --- Will create an Epub version of the novel bioshifter in the current directory.  
* ``royal_road_archiver https://www.royalroad.com/fiction/59450/bioshifter markdown --no-image-tags`` --- Will create a markdown version of the novel bioshifter in the current directory, removing image tags from the markdown file.
* ``royal_road_archiver https://www.royalroad.com/fiction/59450/bioshifter Downloads epub`` --- Will create an Epub version of the novel bioshifter in my Downloads folder.
* Look the commands are all documented in the program just running ``royal_road_archiver help`` will show you everything.

Enjoy.
