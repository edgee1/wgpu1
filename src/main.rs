mod lib;
use lib::run;
use pollster;


fn main() {
    pollster::block_on(run());
}
