/*=============================================================================
* RUST parser for CSP instances represented in XCSP3 Format
*
* Copyright (c) 2026 xcsp.org (contact @ xcsp.org)
*
* Based on the original Rust parser proposed in https://github.com/luhanzhen/xcsp3-rust
* by Luhan Zhen (zhenlh20@mails.jlu.edu.cn)
*
* Permission is hereby granted, free of charge, to any person obtaining a copy
* of this software and associated documentation files (the "Software"), to deal
* in the Software without restriction, including without limitation the rights
* to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
* copies of the Software, and to permit persons to whom the Software is
* furnished to do so, subject to the following conditions:
*
* The above copyright notice and this permission notice shall be included in
* all copies or substantial portions of the Software.
*
* THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
* IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
* FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
* AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
* LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
* OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
* THE SOFTWARE.
*=============================================================================
*/

pub mod xcsp3_core {
    use crate::constraints::xall_different::xcsp3_core::XAllDifferent;
    use crate::constraints::xall_different_except::xcsp3_core::XAllDifferentExcept;
    use crate::constraints::xall_different_list::xcsp3_core::XAllDifferentList;
    use crate::constraints::xall_different_matrix::xcsp3_core::XAllDifferentMatrix;
    use crate::constraints::xall_equal::xcsp3_core::XAllEqual;
    use crate::constraints::xbinpacking::xcsp3_core::XBinpacking;
    use crate::constraints::xcardinality::xcsp3_core::XCardinality;
    use crate::constraints::xchannel::xcsp3_core::XChannel;
    use crate::constraints::xcircuit::xcsp3_core::XCircuit;
    use crate::constraints::xclause::xcsp3_core::XClause;
    use crate::constraints::xconstraint_trait::xcsp3_core::XConstraintUnfold;
    use crate::constraints::xcount::xcsp3_core::XCount;
    use crate::constraints::xcumulative::xcsp3_core::XCumulative;
    use crate::constraints::xelement::xcsp3_core::XElement;
    use crate::constraints::xelement_matrix::xcsp3_core::XElementMatrix;
    use crate::constraints::xextension::xcsp3_core::XExtension;
    use crate::constraints::xgroup::xcsp3_core::XGroup;
    use crate::constraints::xinstantiation::xcsp3_core::XInstantiation;
    use crate::constraints::xintension::xcsp3_core::XIntention;
    use crate::constraints::xknapsack::xcsp3_core::XKnapsack;
    use crate::constraints::xlex::xcsp3_core::XLex;
    use crate::constraints::xlex_matrix::xcsp3_core::XLexMatrix;
    use crate::constraints::xmax_min::xcsp3_core::XMaxMin;
    use crate::constraints::xmax_min_arg::xcsp3_core::XMaxMinArg;
    use crate::constraints::xmdd::xcsp3_core::XMdd;
    use crate::constraints::xn_values::xcsp3_core::XNValues;
    use crate::constraints::xno_overlap::xcsp3_core::XNoOverlap;
    use crate::constraints::xno_overlap_k_dimensional::xcsp3_core::XNoOverlapKDim;
    use crate::constraints::xordered::xcsp3_core::XOrdered;
    use crate::constraints::xprecedence::xcsp3_core::XPrecedence;
    use crate::constraints::xregular::xcsp3_core::XRegular;
    use crate::constraints::xslide::xcsp3_core::XSlide;
    use crate::constraints::xstretch::xcsp3_core::XStretch;
    use crate::constraints::xsum::xcsp3_core::XSum;
    use crate::data_structs::xint_val_var::xcsp3_core::XVarVal;

    #[derive(Clone)]
    pub enum XConstraintType<'a> {
        XExtension(XExtension<'a>),
        XAllDifferent(XAllDifferent<'a>),
        XAllDifferentList(XAllDifferentList<'a>),
        XAllDifferentMatrix(XAllDifferentMatrix<'a>),
        XAllDifferentExcept(XAllDifferentExcept<'a>),
        XInstantiation(XInstantiation<'a>),
        XAllEqual(XAllEqual<'a>),
        XOrdered(XOrdered<'a>),
        XRegular(XRegular<'a>),
        XMdd(XMdd<'a>),
        XIntention(XIntention<'a>),
        XGroup(XGroup<'a>),
        XSum(XSum<'a>),
        XMaximum(XMaxMin<'a>),
        XMinimum(XMaxMin<'a>),
        XMaximumArg(XMaxMinArg<'a>),
        XMinimumArg(XMaxMinArg<'a>),
        XElement(XElement<'a>),
        XElementMatrix(XElementMatrix<'a>),
        XSlide(XSlide<'a>),
        XCount(XCount<'a>),
        XNValues(XNValues<'a>),
        XCardinality(XCardinality<'a>),
        XChannel(XChannel<'a>),
        XCumulative(XCumulative<'a>),
        XNoOverlap(XNoOverlap<'a>),
        XStretch(XStretch<'a>),
        XNoOverlapKDim(XNoOverlapKDim<'a>),
        XCircuit(XCircuit<'a>),
        XClause(XClause<'a>),
        XPrecedence(XPrecedence<'a>),
        XKnapsack(XKnapsack<'a>),
        XBinpacking(XBinpacking<'a>),
        XLex(XLex<'a>),
        XLexMatrix(XLexMatrix<'a>),
    }

    impl<'a> XConstraintUnfold for XConstraintType<'a> {
        fn extract_parameters(&mut self, arg: &[XVarVal]) {
            macro_rules! dispatch {
            ($($variant:ident),* $(,)?) => {
                match self {
                    $(XConstraintType::$variant(inner) => inner.extract_parameters(arg),)*
                    _ => todo!()
                }
            };
        }

            dispatch!(
                XAllDifferent,
                XAllEqual,
                XAllDifferentExcept,
                XCardinality,
                XCount,
                XCumulative,
                XInstantiation,
                XMaximum,
                XMinimum,
                XMdd,
                XNValues,
                XNoOverlap,
                XOrdered,
                XRegular,
                XSum,
                XCircuit,
                XPrecedence,
                XAllDifferentList,
                XMaximumArg,
                XMinimumArg,
                XElement,
                XElementMatrix,
                XChannel,
                XKnapsack,
                XBinpacking,
                XLex,
                XIntention,
                XExtension,
            );
        }

        fn max_args_used(&mut self) -> i32 {
            macro_rules! dispatch {
            ($($variant:ident),* $(,)?) => {
                match self {
                    $(XConstraintType::$variant(inner) => inner.max_args_used(),)*
                    _ => todo!()
                }
            }
        }

            dispatch!(
                XAllDifferent,
                XAllEqual,
                XAllDifferentExcept,
                XCardinality,
                XCount,
                XCumulative,
                XInstantiation,
                XMaximum,
                XMinimum,
                XMdd,
                XNValues,
                XNoOverlap,
                XOrdered,
                XRegular,
                XSum,
                XCircuit,
                XPrecedence,
                XAllDifferentList,
                XMaximumArg,
                XMinimumArg,
                XElement,
                XElementMatrix,
                XChannel,
                XKnapsack,
                XBinpacking,
                XLex,
                XIntention,
                XExtension,
            )
        }
    }
}
