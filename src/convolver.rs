use crate::traits::{TBlockRing, TIRData};
use crate::block_ring::BlockRing;
use crate::ir_data::IRData;

struct Convolver<I : TIRData = IRData, B : TBlockRing = BlockRing>{
    ir_data : I,
    block_ring : B
}


impl Convolver{

}
