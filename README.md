## Graphic Pathfinding


![Main](https://github.com/raybritton/graphical-pathfinding/workflows/Main/badge.svg?branch=master)
[![dependency status](https://deps.rs/repo/github/raybritton/graphical-pathfinding/status.svg)](https://deps.rs/repo/github/raybritton/graphical-pathfinding)


(Only has A* and Dijkstra at the moment)

### This should be run in release mode

#### Keys
* Anytime:
  * **ESC** or **Q** close
  * **R** restart program
* Map:
  * **Up, Down, Left, Right** to highlight map and variant
  * **Enter/Return** to select
* Algorithms, diagonals and heuristics:
  * **Up, Down** to highlight mode
  * **Enter/Return** to select
* Runner:
  * **[** faster
  * **]** slower
  * **p** toggle manual mode
  * **space** advance one tick in manual mode 
  

  
  
  ![Screenshot of runner screen](https://raw.githubusercontent.com/raybritton/graphical-pathfinding/master/screenshot.png)
  ![Legend](https://raw.githubusercontent.com/raybritton/graphical-pathfinding/master/palette.png)


#### Map format

Each map file must have 17 lines starting with `M` followed by 32 digits (`0-9`). The digits represent the cost of that tile: 0 being the lowest, 8 the highest and 9 being impassable. 

Then at least one pair of start end co-ords, which consist of a line starting with an `S` then co-ords of the start point, i.e. `4,5` then another line starting with an `E` with a set of different co-ords. You can have as many different pairs as needed.
