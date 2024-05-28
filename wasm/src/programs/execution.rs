// Copyright (C) 2019-2023 Aleo Systems Inc.
// This file is part of the Aleo SDK library.

// The Aleo SDK library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The Aleo SDK library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the Aleo SDK library. If not, see <https://www.gnu.org/licenses/>.

pub use super::*;
use crate::log;
use std::{ops::Deref, str::FromStr};
use wasm_bindgen::prelude::wasm_bindgen;

use crate::types::native::{
    CurrentNetwork,
    ExecutionNative,
    IdentifierNative,
    ProcessNative,
    ProgramID,
    VerifyingKeyNative,
};

/// Execution of an Aleo program.
#[wasm_bindgen]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Execution(ExecutionNative);

#[wasm_bindgen]
impl Execution {
    /// Returns the string representation of the execution.
    #[wasm_bindgen(js_name = "toString")]
    #[allow(clippy::inherent_to_string)]
    pub fn to_string(&self) -> String {
        self.0.to_string()
    }

    /// Creates an execution object from a string representation of an execution.
    #[wasm_bindgen(js_name = "fromString")]
    pub fn from_string(execution: &str) -> Result<Execution, String> {
        Ok(Self(ExecutionNative::from_str(execution).map_err(|e| e.to_string())?))
    }
}

impl From<ExecutionNative> for Execution {
    fn from(native: ExecutionNative) -> Self {
        Self(native)
    }
}

impl From<Execution> for ExecutionNative {
    fn from(execution: Execution) -> Self {
        execution.0
    }
}

impl Deref for Execution {
    type Target = ExecutionNative;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Verify an execution with a single function and a single transition. Executions with multiple
/// transitions or functions will fail to verify. Also, this does not verify that the state root of
/// the execution is included in the Aleo Network ledger.
///
/// @param {Execution} execution The function execution to verify
/// @param {VerifyingKey} verifying_key The verifying key for the function
/// @param {Program} program The program that the function execution belongs to
/// @param {String} function_id The name of the function that was executed
/// @returns {boolean} True if the execution is valid, false otherwise
#[wasm_bindgen(js_name = "verifyFunctionExecution")]
pub fn verify_function_execution(
    execution: &Execution,
    verifying_key: &VerifyingKey,
    program: &Program,
    function_id: &str,
) -> Result<bool, String> {
    let function = IdentifierNative::from_str(function_id).map_err(|e| e.to_string())?;
    let program_id = ProgramID::<CurrentNetwork>::from_str(&program.id()).unwrap();
    let mut process = ProcessNative::load_web().map_err(|e| e.to_string())?;
    if &program.id() != "credits.aleo" {
        process.add_program(program).map_err(|e| e.to_string())?;
    }
    process
        .insert_verifying_key(&program_id, &function, VerifyingKeyNative::from(verifying_key))
        .map_err(|e| e.to_string())?;
    process.verify_execution(execution).map_or(Ok(false), |p| Ok(true))
}

// TODO: Re-add tests after initial SDK release
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use wasm_bindgen_test::*;
//
//     const EXECUTION: &str = r#"{"transitions":[{"id":"au1y8g2y8fvum3artfj3wqz9yps2u30s4zk6cssaqhg5yj9tpx8eszqn7erjp","program":"credits.aleo","function":"transfer_public","inputs":[{"type":"public","id":"5728323912539873901463744923106579772849880966232653596587848724800440854698field","value":"aleo12tksdptp7hvxly8tkm3um08fvf53qpehsgdgqfvy9pe3sewcq5ysjg5myy"},{"type":"public","id":"5747305006533197425360059474083535488071694934708374669191415686930805801476field","value":"10000000000000u64"}],"outputs":[{"type":"future","id":"8260514824634633030701492989170989242596854865439089259126466320867563479449field","value":"{\n  program_id: credits.aleo,\n  function_name: transfer_public,\n  arguments: [\n    aleo12tksdptp7hvxly8tkm3um08fvf53qpehsgdgqfvy9pe3sewcq5ysjg5myy,\n    aleo12tksdptp7hvxly8tkm3um08fvf53qpehsgdgqfvy9pe3sewcq5ysjg5myy,\n    10000000000000u64\n  ]\n}"}],"tpk":"1443791236595734816721808594797682198514246646666543554175320023762598050716group","tcm":"3124189233089316494525019740495817071312710677756667238858579956966409322827field","scm":"7147560898329539385845695555565243574844127625207818002552572984487942831128field"}],"global_state_root":"sr1ekees06ce437zyrpy3xryal7wpfsw2zlsvwrr0rrfv3ywc8ehcrs6mjr2t","proof":"proof1qyqsqqqqqqqqqqqpqqqqqqqqqqqdxjp0rrnrukwx66zxzzxv3jtqwqar5qakf590hyt0653vnuqfy0txjz34l3f9747w2v6apf86e9vpq80vgpkfu0663yee9n73g4gym0e9kzg4t7m45rf6336f40t9eqqjhpc0hd3cvsry6r8q20g40qe3lqdr9efumsuew2m54zygyruuej23kzwppzvy4wm0xjd9pmplgz2tt8gdvdvrlggjlhp4jna2w54pqjqjhtj4glv7j97nyad7wsfl0qj9ynpjn7gxc0ypzl2gg36qkn0kj0vtdcr4w28h6g6hr90y5jjfstsqgd7myt58ltve9z6v6p99awkgf4ex9eckv0x0vs8g4wyjf64p4897udd54ceg4km60asqhfqx9szcq2dlhwd7pf6ke53v8st4eamxdk9plsu9e0ueux2tknp73hup28hnjzpjlwje6438ehq6s36fz9tesqzww57da0svmvu7zc72w78pdqvk22w2nur28rvx0vvcvck8acpqrfewcu9xltv5hmzmlnmp8k7lkqpy2ynefnle3qss9d265tz2qu8mkwjq2la5nygvm56jvp0fdjazat0rdt29qjqvefjlq8retaqhqkqzu8nn2hhljhd3nljk4alq4hw5qwhdmz6884sezm8gmcw89w0mhqktyanzlk8p3qfczfk3aawn53up6kus7ygccazpk08ajh3s4t09ajpuz32n7sk2267z292pu40t8syccstzar65gl6hjkn02agruzdp5hlaryl20pvtl45sndxwyhykxzvh8xy07h03vr7qdcnspphqjsrdexvcafj4h00p3tswedj4twgcqca89x062c4ych4ky8dtdvvan5wazqwv3ecqg8qfvj3r842pe2ys8v9x4wz394tnjhxtwwl6g8my88mytt88hhw92ejjuu70375s4eqzpsatd3943y05uuy55nrunntsfx5lrhx8kj46ypj2x6mhgthsqvr43ykrtnpngjdxwqrgcwsh4tpynmrasgp42ffqr9kpyqq2nqjw5r9tddp92d9xzf7x2tu2j2lrruyarxwga3234c0jdfu6lxhezq7upnug7vvx9zq07rv5j3g03vqusptykdy7k7fyfparzgc5gr7maggsl6u3k2xz45ww687prfk8gjcfrmuklax0j9mfpxhr4zy00znh33cfqvqqqqqqqqqqq0q60aqmmdesrr8dm7s6zrqtgkpe6dmptmrl8kj2vna7xdjxqq0g46js4pcy8y50n29g9c3tfzu7syqpluwspydz4jt02gdn5zfcm3kggexkj6lcxe40fm948rdpknl8qq98w452mrkhu4heuxde5gvernsqq9juwjerdc0awzapcvzn49k5kjqv28l94ysud9vr67ukqka7g3epz9l4ng33hdw7xuytxdw4cnknkrld3u2pltxnal2vlwnpjmlv245rqqhqsa8puprxnnmfk70cavtaqqqqw7jtdw"}"#;
//     const TRANSFER_PUBLIC_VERIFIER: &str = "verifier1qygqqqqqqqqqqqp3xqqqqqqqqqqzvvqqqqqqqqqq23hqqqqqqqqqqau5qqqqqqqqqq5yzqqqqqqqqqqvqqqqqqqqqqqz2rh4q6m4u0ycv2z5qx95echpdcsktezkr2j9cvff0dngp45jfqggm8q3578nkhjudslm2rdpsgcpks4ulquyqrtd978zvj65pxmhkudyjtj005h66jcnwg3f6mdqqaqjed3avcz599kth8a3nak0tftsrk4hczcdvlmrdnzsa6rfppy72flsrhdhn6npxfxt2rrudk8jrk5fefkawhhrf0psccp6l9akckaps898v5mk7vprkx90gg798d6j5tvvtma0r9phq3jndan5rkwv8wmkngeha3pzrjzslkt8ct9umm4wfq8dhnpdv8m4plq70c3d6wxs4l3cv4gyqhgjtwfuydc5fflulwgjvdtaxxlmpf0l5n800jn2lwnt5kqqtdsx2lvzl3kw3ns5hnsu4jzg9ejptg7n7r7d9dvqsrvldw9paeq86hjxeaac6typemynwzt8w3vq3e27l4swd9aqxaas7zsy5mw7ftnjct9jq6t9rdq0kx8vjag77nt0z894xqkrfp27hehk83nfcg7cpjcc7f8mznga33xp36seafxpn26rq4w5l9uawtx02mzpq9kuyyx7nvhtad0mxk5su659xx5wv3yhqqy3577kwc40spvp9dst59ap2ll5uuq7qaauy9vy5yvrjd77c443evxrwfsee8jg5hvt6f9xaupjzq2cmuuwdvjs0qtxmnwjmdepu5979qy4nhqs63zljatt7kkggddxctt74crayt4djvnh3u6tjzm8wyqpc7nf0r0k9mp34x9musgdqegkmscey50ckyehm67c5tcdlawkh6my2v3xzwaug7c4y9xyw0dfkqup24jdckk5wxkyfd8rsp6h220dj786t54fyj59ehtq0gyut6lcattumgdss4kkchrdp7f5sjgcu7ycpzq0d47m6xe3yh2q5q76fa20lm46pe8fcd9yqcrxduhkdqfe4c9pw3pqqqqqqqqqq0qvtsl";
//
//     #[wasm_bindgen_test]
//     fn test_execution_verification() {
//         let execution = Execution::from_string(EXECUTION).unwrap();
//         let verifying_key = VerifyingKey::from_string(TRANSFER_PUBLIC_VERIFIER).unwrap();
//         assert!(
//             verify_function_execution(&execution, &verifying_key, &Program::get_credits_program(), "transfer_public")
//                 .unwrap()
//         );
//     }
// }
