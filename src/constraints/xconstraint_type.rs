/*=============================================================================
* parser for CSP instances represented in XCSP3 Format
*
* Copyright (c) 2023 xcsp.org (contact @ xcsp.org)
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

/*
* <p>@project_name: xcsp3-rust
* </p>
* <p>@author: luhan zhen
* </p>
* <p>@date:  2023/7/14 18:56
* </p>
* <p>@email: zhenlh20@mails.jlu.edu.cn
* </p>
* <p>@version: 1.0
* </p>
 * <p>@description: 1.0
* </p>
 */

pub mod xcsp3_core {
    use crate::constraints::xall_different::xcsp3_core::XAllDifferent;
    use crate::constraints::xall_different_except::xcsp3_core::XAllDifferentExcept;
    use crate::constraints::xall_equal::xcsp3_core::XAllEqual;
    use crate::constraints::xcardinality::xcsp3_core::XCardinality;
    use crate::constraints::xchannel::xcsp3_core::XChannel;
    use crate::constraints::xconstraint_trait::xcsp3_core::XConstraintUnfold;
    use crate::constraints::xcount::xcsp3_core::XCount;
    use crate::constraints::xcumulative::xcsp3_core::XCumulative;
    use crate::constraints::xelement::xcsp3_core::XElement;
    use crate::constraints::xextension::xcsp3_core::XExtension;
    use crate::constraints::xgroup::xcsp3_core::XGroup;
    use crate::constraints::xinstantiation::xcsp3_core::XInstantiation;
    use crate::constraints::xintension::xcsp3_core::XIntention;
    use crate::constraints::xmax_min::xcsp3_core::XMaxMin;
    use crate::constraints::xmdd::xcsp3_core::XMdd;
    use crate::constraints::xn_values::xcsp3_core::XNValues;
    use crate::constraints::xno_overlap::xcsp3_core::XNoOverlap;
    use crate::constraints::xno_overlap_k_dimensional::xcsp3_core::XNoOverlapKDim;
    use crate::constraints::xordered::xcsp3_core::XOrdered;
    use crate::constraints::xregular::xcsp3_core::XRegular;
    use crate::constraints::xslide::xcsp3_core::XSlide;
    use crate::constraints::xstretch::xcsp3_core::XStretch;
    use crate::constraints::xsum::xcsp3_core::XSum;
    use crate::data_structs::xint_val_var::xcsp3_core::XVarVal;
    use crate::errors::xcsp3error::xcsp3_core::Xcsp3Error;
    use std::fmt::{Display, Formatter};

    #[derive(Clone)]
    pub enum XConstraintType<'a> {
        XConstraintNone(Xcsp3Error),
        XExtension(XExtension<'a>),
        XAllDifferent(XAllDifferent<'a>),
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
        XElement(XElement<'a>),
        XSlide(XSlide<'a>),
        XCount(XCount<'a>),
        XNValues(XNValues<'a>),
        XCardinality(XCardinality<'a>),
        XChannel(XChannel<'a>),
        XCumulative(XCumulative<'a>),
        XNoOverlap(XNoOverlap<'a>),
        XStretch(XStretch<'a>),
        XNoOverlapKDim(XNoOverlapKDim<'a>),
    }

    impl<'a> XConstraintUnfold for XConstraintType<'a> {
        fn extract_parameters(&mut self, arg: &[XVarVal]) {
            macro_rules! dispatch {
            ($($variant:ident),* $(,)?) => {
                match self {
                    $(XConstraintType::$variant(inner) => inner.extract_parameters(arg),)*
                    XConstraintType::XConstraintNone(_) => {
                        // Nothing to extract from an error state
                    },
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
                XInstantiation,
                XMaximum,
                XMinimum,
                XMdd,
                XNValues,
                XNoOverlap,
                XOrdered,
                XRegular,
                XSum
            );
        }
    }
}
