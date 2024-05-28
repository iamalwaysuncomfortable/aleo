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

use crate::types::native::TransactionNative;

use std::str::FromStr;
use wasm_bindgen::prelude::wasm_bindgen;

/// Webassembly Representation of an Aleo transaction
///
/// This object is created when generating an on-chain function deployment or execution and is the
/// object that should be submitted to the Aleo Network in order to deploy or execute a function.
#[wasm_bindgen]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Transaction(TransactionNative);

#[wasm_bindgen]
impl Transaction {
    /// Create a transaction from a string
    ///
    /// @param {string} transaction String representation of a transaction
    /// @returns {Transaction | Error}
    #[wasm_bindgen(js_name = fromString)]
    pub fn from_string(transaction: &str) -> Result<Transaction, String> {
        Transaction::from_str(transaction)
    }

    /// Get the transaction as a string. If you want to submit this transaction to the Aleo Network
    /// this function will create the string that should be submitted in the `POST` data.
    ///
    /// @returns {string} String representation of the transaction
    #[wasm_bindgen(js_name = toString)]
    #[allow(clippy::inherent_to_string)]
    pub fn to_string(&self) -> String {
        self.0.to_string()
    }

    /// Get the id of the transaction. This is the merkle root of the transaction's inclusion proof.
    ///
    /// This value can be used to query the status of the transaction on the Aleo Network to see
    /// if it was successful. If successful, the transaction will be included in a block and this
    /// value can be used to lookup the transaction data on-chain.
    ///
    /// @returns {string} Transaction id
    #[wasm_bindgen(js_name = transactionId)]
    pub fn transaction_id(&self) -> String {
        self.0.id().to_string()
    }

    /// Get the type of the transaction (will return "deploy" or "execute")
    ///
    /// @returns {string} Transaction type
    #[wasm_bindgen(js_name = transactionType)]
    pub fn transaction_type(&self) -> String {
        match &self.0 {
            TransactionNative::Deploy(..) => "deploy".to_string(),
            TransactionNative::Execute(..) => "execute".to_string(),
            TransactionNative::Fee(..) => "fee".to_string(),
        }
    }
}

impl From<Transaction> for TransactionNative {
    fn from(transaction: Transaction) -> Self {
        transaction.0
    }
}

impl From<TransactionNative> for Transaction {
    fn from(transaction: TransactionNative) -> Self {
        Self(transaction)
    }
}

impl FromStr for Transaction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(TransactionNative::from_str(s).map_err(|e| e.to_string())?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;
    // const TRANSACTION_STRING: &str = "{\"type\":\"execute\",\"id\":\"at1rh04nydu2m07n9wm3pugmlaqh7775lfuawa86ed4eymv2q9wkc9qahtx66\",\"execution\":{\"transitions\":[{\"id\":\"au1xe07pjnw6970k9lh0rfvpdnudcz0gcyy5qmv2efp3qrdxkkaj5rseklkfk\",\"program\":\"credits.aleo\",\"function\":\"transfer_public\",\"inputs\":[{\"type\":\"public\",\"id\":\"6830040130268084683056203786650856838291629627526850542328121029117462649106field\",\"value\":\"aleo1q6qstg8q8shwqf5m6q5fcenuwsdqsvp4hhsgfnx5chzjm3secyzqt9mxm8\"},{\"type\":\"public\",\"id\":\"3522622156280992546879723962866054193411134839313162974822034464277507937156field\",\"value\":\"1u64\"}],\"outputs\":[{\"type\":\"future\",\"id\":\"6287946476679554718269040652777030908815963134664267470652690289159774741065field\",\"value\":\"{\\n  program_id: credits.aleo,\\n  function_name: transfer_public,\\n  arguments: [\\n    aleo1q6qstg8q8shwqf5m6q5fcenuwsdqsvp4hhsgfnx5chzjm3secyzqt9mxm8,\\n    aleo1q6qstg8q8shwqf5m6q5fcenuwsdqsvp4hhsgfnx5chzjm3secyzqt9mxm8,\\n    1u64\\n  ]\\n}\"}],\"tpk\":\"1124897318163588088766079717854473596955076479615330099205126740598806373414group\",\"tcm\":\"5379517959399780344431681060960827894902462418861353186658003065156167347920field\"}],\"global_state_root\":\"sr1tml46c266j4gzv9qpk0adkt6tkl4mjq7s7supuy8dmv9lq09j5zsd5eqsz\",\"proof\":\"proof1qyqsqqqqqqqqqqqpqqqqqqqqqqq9eqfncmzufz24n5xvfk2yy2lm7k0jh2y23yj5ssekln7h2nmlc62mnjwe794rn5dxwwf7unaamfyqqxzhaqnm3xws740w8gwt3dt22r5l43xa9rhn6yc0vpuu46mal3a86n3qmc8yegeh8afyetmz7rs8mq8766v6rryrnhnhl8xudl3tr7rk50f0lrz36cjwp0vpg46fzq4wv9n3eglkn9ztx4kzhh9d0wmqgcqvv2e6lrnaqp9cafaxjh88pzfjn26vyq3y50hazf9c9ysc84x33mn4wculvu67z2utduq5qyy933qqtn7u5rtsztmtakuu2japhf7qcvrc663vkuk9s0twufhh42d2kk3ukf00290jxqe9qnfwr3txz05spv7tp88e7dduldq5wwulae6wm3nztzmzdjrypfz08awuvkzuale9h96hy8nyjt2znntu20c4xemlsyqpfn6ce0sv5nn5shxx2up8kw9xtyle3pcyaum9hsw29ctqcjqmn53j7dxy6ep37cfvnflxqctpuqqgtgss9tp5rzg4vp46fw8nsjztdum9xm2xp0d8hglr2v8fuyh38afsw0kymhn2lznaag7cwud0guqtpdn2l2zgn4sjg7n0gdd0ueg5ujeydmqkxx8dp9a4g456q4jvukjt2cycuvef5slqt3hwnuh6ez5qys785f6xw8xsc5ns6ee3la7rf3p24mkpeakd5ay73q30m3qezux2xzqv35zy8jclv7lxpvcnej8tkstf00fzh9l44q382hpt8eejyp3vs3pq2n3n2e0eq30zatqyrvqsqs4cllynrcytc6v9seuqgtdnzy5rr3vcwwhlrxzm2h66e9q4z94r7zpnkm6xt4yermvys6twaw6sncwt5x64qjdnatddjpeh97uszkvamu6kmltu2unnq2mq4kverfsg8ncpvkhvre77yjhgmw5nevw2az0s2dwr6navrchn7pdwkmmjcu9dedn40jacflfeld7agznzjw3cpwewyhhufu49l5ttjpqrcpl3yzn2m3h3mgq5ea4xedf8370lmmr4ansr60x5d0qwrx08n6r8qe6vq2jlk5t8fey0mcgteef0hxe84vm5khehwjr9vu7p839jpysctaz3z88zcum9nw6z04gqvj3dqvldndldatwknwu4plnlpsqxhg5x8qc0qvqqqqqqqqqqqf57wtv2gucqnx8n0ejkhtywfmxrvewes27sr0ng67f7900w4kga6ztwnzhvtpd4rv0qqhjfmkwssqqvzr0sf8p6hsda4wgak42gtdcxfkf8xfywmdpgecqhxcptdrtvluv9adsn8vc5vwds4kd54thnaggqqxz3799lcez0f8v2xencv2z76fwvgp52wrnh5qyjtteckn2nhlhq8e60eq358pw2lezf74flec2wp4e8gvk5xtrwwy36j36axmy9pmh9kwa9cnsykxzwx38kdtqhnqytsyqqefuv9d\"},\"fee\":{\"transition\":{\"id\":\"au1etgu5md0jd6r3ddyyyg2sr63r5t07tpyaz452redx4rkzjq0dsqqhpp390\",\"program\":\"credits.aleo\",\"function\":\"fee_public\",\"inputs\":[{\"type\":\"public\",\"id\":\"6202819827443625105167501394613513736747919402680330317066933896152810974533field\",\"value\":\"263388u64\"},{\"type\":\"public\",\"id\":\"562300717734796433896686862094601391435687829906952149841892194225660591975field\",\"value\":\"10000u64\"},{\"type\":\"public\",\"id\":\"7883237601094350949043895657366086123587113448531301549046231111645620087265field\",\"value\":\"2003683512649368822747780913503982093177226997476011687299266369052228066725field\"}],\"outputs\":[{\"type\":\"future\",\"id\":\"4003143084174123056047141027259441734516045162674697571360224656380933228034field\",\"value\":\"{\\n  program_id: credits.aleo,\\n  function_name: fee_public,\\n  arguments: [\\n    aleo1q6qstg8q8shwqf5m6q5fcenuwsdqsvp4hhsgfnx5chzjm3secyzqt9mxm8,\\n    273388u64\\n  ]\\n}\"}],\"tpk\":\"6391092584190750271169179247921812490330238842823180033518604373506188720330group\",\"tcm\":\"1287501176934805652296722632500653287206667978232913581134582827412687172453field\"},\"global_state_root\":\"sr1tml46c266j4gzv9qpk0adkt6tkl4mjq7s7supuy8dmv9lq09j5zsd5eqsz\",\"proof\":\"proof1qyqsqqqqqqqqqqqpqqqqqqqqqqq0vc7dct0xgm89qpdtq9cljzp27k4u0fa0nz0hd8nqlsgjkcj08evn2840dj4k4ygh8tlce383kmqqq9suy0upadgravumuq32nne0zd6673xwj74vunuugsyv40q5ypfjkmdt7t2dsl60ahqpquh5scecyqq6xcz9v72n84l2xl62c79vdc6h99vm5z4ag2mzt5j8lng4dfscxlf9ka067nyu05yh0xq46s7fhzqtf3eadr4kd4fpqqgmkzaj348juxuxqjexv5xtlyl38p6cu5lnn4wylg0w3a02wj4atm5k8rumsaup6dsk6lg9l2r7y3x0cjgw3fc9c9pqnkm86p8udhsjrxxmtp2gaevawyrezqk58x2hwfrgs83f2zhqqhrlxrq6642xjvj02mcj9llucyu234zd04m3hpl05hzu4fx2cuu8wxh74uw0h4dsq8l7g9qm3cn7s87d6drs29j4rexrfwjlnn4hsuvt09elcjtju9ll44vkv004hnmc3rx2gsjwxqgrhxc0w0prntkvzqpmtk33a6fq0v0adeagnc30c76va997pfe37d48z2yvjg9npnvu7xq56tzu32gsnvq8mxnu86zvj5qt2w268h4k9f20smjy2256n6tl7v3m0zxvaqjg8qgwvucwr5r0eufxn7h48eqx6zxawhr5l03jtkgqccqkn5qc67ds9hwn8xrh8hwahu8xf8tykuaq0m7kqm4ww3uzncqtnjxnxgswkj8zecqjvj0p5cs9muejvzgfr6dfr6ekmxm62quecp5q40j9n5vaflwjmwmme5n29ejnf4gfq6yfjt0gdf95mecszfprqjuyr7hvn93qk5qlhcdve9e9sh5q42wj5y74v50jycsp5zuy76es7669stqf3wl0su6zkd44yuwu8tglqdk62mmmjx9590qxcftuqzsthvky3pp0pzpmfelawh0snx439xq5m47zt4u4tf3qga0l5zasvs95n5yulrumfj5uyxs3ssnf47vnh7v2xxmetc46gml2hxwtzsf5uykwdg36l5xxfe9zyn3fzv58vk4hddudkm93plkkqszl956aat24q55tfd5cvmxyy4vu4pg7lvrztf6zm8mpa74ruy8f7vzescnct06qwtf06tygpqya32ehfh23crr9s0s6g8dvzwaq9k26jq0qqk287ps8qvqqqqqqqqqqqn3e0c3qnwqq9pcuczyc53yy7qrya5gapevrkpxf8tm7mmeg8ujh88ru7lj24j0d9almx5uherfysqqwdy8z5kr7ky8qhz4z4ze8qx8mlcd5ksxs7vdpktn2nz3437yejegtu0k6u68up4spsfhgmt9z03vqq9fujl8u73uquc8xjpqqpvwr8dsf4zy3s5p2n43w7gh6kte0luxs76pmkhns42fkktusftt4ymyw083c580awfx97zeq76wyp3x2fn3lmeepwpkva2ms9u757yp2mj62qqqqsuydxg\"}}";
    const TESTNET_TRANSACTION_STRING: &str = "{\"type\":\"execute\",\"id\":\"at1jtk8du0xnt5e92v4hxz8ckys22x5s8dwc5y3uejxwrkev7xqny8sh7wz0e\",\"execution\":{\"transitions\":[{\"id\":\"au1y8g2y8fvum3artfj3wqz9yps2u30s4zk6cssaqhg5yj9tpx8eszqn7erjp\",\"program\":\"credits.aleo\",\"function\":\"transfer_public\",\"inputs\":[{\"type\":\"public\",\"id\":\"5728323912539873901463744923106579772849880966232653596587848724800440854698field\",\"value\":\"aleo12tksdptp7hvxly8tkm3um08fvf53qpehsgdgqfvy9pe3sewcq5ysjg5myy\"},{\"type\":\"public\",\"id\":\"5747305006533197425360059474083535488071694934708374669191415686930805801476field\",\"value\":\"10000000000000u64\"}],\"outputs\":[{\"type\":\"future\",\"id\":\"8260514824634633030701492989170989242596854865439089259126466320867563479449field\",\"value\":\"{\\n  program_id: credits.aleo,\\n  function_name: transfer_public,\\n  arguments: [\\n    aleo12tksdptp7hvxly8tkm3um08fvf53qpehsgdgqfvy9pe3sewcq5ysjg5myy,\\n    aleo12tksdptp7hvxly8tkm3um08fvf53qpehsgdgqfvy9pe3sewcq5ysjg5myy,\\n    10000000000000u64\\n  ]\\n}\"}],\"tpk\":\"1443791236595734816721808594797682198514246646666543554175320023762598050716group\",\"tcm\":\"3124189233089316494525019740495817071312710677756667238858579956966409322827field\",\"scm\":\"7147560898329539385845695555565243574844127625207818002552572984487942831128field\"}],\"global_state_root\":\"sr1ekees06ce437zyrpy3xryal7wpfsw2zlsvwrr0rrfv3ywc8ehcrs6mjr2t\",\"proof\":\"proof1qyqsqqqqqqqqqqqpqqqqqqqqqqqdxjp0rrnrukwx66zxzzxv3jtqwqar5qakf590hyt0653vnuqfy0txjz34l3f9747w2v6apf86e9vpq80vgpkfu0663yee9n73g4gym0e9kzg4t7m45rf6336f40t9eqqjhpc0hd3cvsry6r8q20g40qe3lqdr9efumsuew2m54zygyruuej23kzwppzvy4wm0xjd9pmplgz2tt8gdvdvrlggjlhp4jna2w54pqjqjhtj4glv7j97nyad7wsfl0qj9ynpjn7gxc0ypzl2gg36qkn0kj0vtdcr4w28h6g6hr90y5jjfstsqgd7myt58ltve9z6v6p99awkgf4ex9eckv0x0vs8g4wyjf64p4897udd54ceg4km60asqhfqx9szcq2dlhwd7pf6ke53v8st4eamxdk9plsu9e0ueux2tknp73hup28hnjzpjlwje6438ehq6s36fz9tesqzww57da0svmvu7zc72w78pdqvk22w2nur28rvx0vvcvck8acpqrfewcu9xltv5hmzmlnmp8k7lkqpy2ynefnle3qss9d265tz2qu8mkwjq2la5nygvm56jvp0fdjazat0rdt29qjqvefjlq8retaqhqkqzu8nn2hhljhd3nljk4alq4hw5qwhdmz6884sezm8gmcw89w0mhqktyanzlk8p3qfczfk3aawn53up6kus7ygccazpk08ajh3s4t09ajpuz32n7sk2267z292pu40t8syccstzar65gl6hjkn02agruzdp5hlaryl20pvtl45sndxwyhykxzvh8xy07h03vr7qdcnspphqjsrdexvcafj4h00p3tswedj4twgcqca89x062c4ych4ky8dtdvvan5wazqwv3ecqg8qfvj3r842pe2ys8v9x4wz394tnjhxtwwl6g8my88mytt88hhw92ejjuu70375s4eqzpsatd3943y05uuy55nrunntsfx5lrhx8kj46ypj2x6mhgthsqvr43ykrtnpngjdxwqrgcwsh4tpynmrasgp42ffqr9kpyqq2nqjw5r9tddp92d9xzf7x2tu2j2lrruyarxwga3234c0jdfu6lxhezq7upnug7vvx9zq07rv5j3g03vqusptykdy7k7fyfparzgc5gr7maggsl6u3k2xz45ww687prfk8gjcfrmuklax0j9mfpxhr4zy00znh33cfqvqqqqqqqqqqq0q60aqmmdesrr8dm7s6zrqtgkpe6dmptmrl8kj2vna7xdjxqq0g46js4pcy8y50n29g9c3tfzu7syqpluwspydz4jt02gdn5zfcm3kggexkj6lcxe40fm948rdpknl8qq98w452mrkhu4heuxde5gvernsqq9juwjerdc0awzapcvzn49k5kjqv28l94ysud9vr67ukqka7g3epz9l4ng33hdw7xuytxdw4cnknkrld3u2pltxnal2vlwnpjmlv245rqqhqsa8puprxnnmfk70cavtaqqqqw7jtdw\"},\"fee\":{\"transition\":{\"id\":\"au18yekadzq2s5yueclgjrpsn6f5nvz3eeak70yuq06uy0n24adyyys8zx0jk\",\"program\":\"credits.aleo\",\"function\":\"fee_public\",\"inputs\":[{\"type\":\"public\",\"id\":\"3080832180601027935910932515052213391777874370190043796269735911647549813403field\",\"value\":\"51060u64\"},{\"type\":\"public\",\"id\":\"7056660245877277213846039495094519674382305799912855297264489254703531489205field\",\"value\":\"0u64\"},{\"type\":\"public\",\"id\":\"3789927067447441635169953860512561549280331988162246271370367448733014580560field\",\"value\":\"650363021925585411164846421962713294200755326234354059965840329019131016063field\"}],\"outputs\":[{\"type\":\"future\",\"id\":\"3699459455385485673291537203077577582216687613837689580466741718132804211508field\",\"value\":\"{\\n  program_id: credits.aleo,\\n  function_name: fee_public,\\n  arguments: [\\n    aleo12tksdptp7hvxly8tkm3um08fvf53qpehsgdgqfvy9pe3sewcq5ysjg5myy,\\n    51060u64\\n  ]\\n}\"}],\"tpk\":\"1902721968283688077927786421906942930560999184584720592100304140820154629718group\",\"tcm\":\"3387410122830265412442587649752217196999381457869829183774696773320911233244field\",\"scm\":\"4938929615252521064092020685272661923677655998269959268215473396901445108922field\"},\"global_state_root\":\"sr1ekees06ce437zyrpy3xryal7wpfsw2zlsvwrr0rrfv3ywc8ehcrs6mjr2t\",\"proof\":\"proof1qyqsqqqqqqqqqqqpqqqqqqqqqqqtrgzk2uqwwsr24fj4eh4rxrx0qwvjvppspeuy2wkr8ff9eczmkgm9w9mm73kkxzsy7954l72rmggqqx3nqehqtx95ravdd77p8pfv9rfneryydcym7fzfr2cxv8rrcuwlwea2l5jnm3xf82q0acssusk9jqv6ru2may6wn7wflvmygxxnpgwvjet45lm3ual5l4drk6jx8a72pzcsdj7y908gmlnpky9ld8y7suqf8crd5qtr80awze8gqrqe4ac6l5xczk8uypnmmy8yk5achh4vu0yjr9ac7j6een7ncydnc258d8vq647j2k85us4n569lzcxkxqvrxv6ltnxs6skem636c20es32zej38v8a059dld6sh272tl9dhnjygzj2ye3pn88ltyyu8cnf9qgztejmjatjupfx5a84tjzrlkx95vzk9jwzgm82wg2u74klmy5ntjhvas9xmn89uslc59a0772emk3t7s6q2hh35pku6ttj9sh2p53442fry2q9wyxtrgkqcxmw7av7e94gv5qrl94lvyt27lttkfrfu8mep4wnd5pc7w4dtymehvv2n9fr949jp0wph2r4gw7exylwrhzxc62p8x7qa6u30ms800fehy0g46nct8yauv0esnxc0ptjz03nvluxxy5ewt4hxhft74v7av0wgxjq8tg2577cqn2qevsv4rsjeqzz2je3y6xv02svpepq2278sm4uln9j3dqppf5qjzdszqclrxj7lmxp0667hmt9zcxhf4tu4nnyvrej7x7u5ckkqsryqump5tj60sgpqe7mw7v3z5la9a0adptcna23twhqheles50edz8wh4c6ucgfws8uu9vquahdx3yyvqfxg7mcfnv2f3a6awnj42v6srylr4tpl3sk88nclsfdqt3k82lqsqw5dts5pajkpgh356gpszcs00tetpmwwk5nu27qrnpx9dzlqp6hvgunlzlkh4wzm3jyzg7m8quxw3herlzr44rme6j2qvrtlp35mc5qpsf4cywd574525f2zg55zgytc57pg26gfavdhznn4w2ra5mwjfvygss5ydv6rwgnfdwwqlmtyphepnm24rawq7772vcryq7dumfl8tnk2hfatx030fcuf66fmu00sqjepf0vff488vefwxamq0zk85j5ndze4y3944fylkg6xq8csqsgzqvqqqqqqqqqqpjm974l05g8e9yhn9k7cnugcdef6grqmjlrjzhapyy8q54gtst3qesupus8xnl52w9vly7c9sh7cqqqrwz048g6lsr356jzmw8j7qrcz6r84lkgql4c3s7k32dsh9ttq4tq4rkkddr077gxe4s96lky2q9upq9peg9lkdx0qlvshprkp5ftj9cdrd2x8mfm5jp9l3fr889u6ghfq6j9kxgcm8pnfagdekgqgat0ts95xw9vpzwva2zq3m8p0h0nxnvhq9ktc3xdckpz3g2tq85aka7tjqyqqe9rze3\"}}";

    const TRANSACTION_ID: &str = "at1jtk8du0xnt5e92v4hxz8ckys22x5s8dwc5y3uejxwrkev7xqny8sh7wz0e";

    #[wasm_bindgen_test]
    fn test_transaction_string_constructor_and_accessor_methods() {
        let transaction = Transaction::from_string(TESTNET_TRANSACTION_STRING).unwrap();
        let transaction_id = transaction.transaction_id();
        let transaction_type = transaction.transaction_type();
        let recovered_string = transaction.to_string();
        assert_eq!(transaction_id, TRANSACTION_ID);
        assert_eq!(transaction_type, "execute");
        assert_eq!(recovered_string, TESTNET_TRANSACTION_STRING);

        // Test to and from round trip
        let transaction_native = TransactionNative::from_str(TESTNET_TRANSACTION_STRING).unwrap();
        let transaction_deconstruction = TransactionNative::from(transaction.clone());
        assert_eq!(transaction_native, transaction_deconstruction);
        let transaction_from_native = Transaction::from(transaction_native);
        assert_eq!(transaction, transaction_from_native);
    }
}
