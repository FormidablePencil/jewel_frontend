use yew::prelude::*;
use yew::{html, Html};
use crate::Route;
use crate::Link;

pub struct MyInvestments {
    // todo - unique name
    name: AttrValue,
    amount: u128,
    earned_percentage: u8,
}

pub fn get_investments() -> Vec<MyInvestments> {
    let investments = vec![
        MyInvestments { name: AttrValue::from("Dennis"), amount: 4234, earned_percentage: 0 },
        MyInvestments { name: AttrValue::from("Daniel A"), amount: 2121, earned_percentage: 0 },
        MyInvestments { name: AttrValue::from("Daniel Ch"), amount: 1039, earned_percentage: 0 },
        MyInvestments { name: AttrValue::from("Ruslan"), amount: 2993, earned_percentage: 0 },
        MyInvestments { name: AttrValue::from("Simon"), amount: 1339, earned_percentage: 0 },
        MyInvestments { name: AttrValue::from("Tim"), amount: 1933, earned_percentage: 0 },
    ];
    investments
}

#[function_component]
pub fn Investments() -> Html {
    let investments = get_investments();

    html! {
        <div class="investments">
            <p>{"My investments"}</p>
        // list of all user's investments
        {
            investments.into_iter().map(|investment| {
                html!{
                    <div class="card" key={investment.name.as_str()}>
                         <Link<Route> classes={classes!("navbar-item")} to={Route::InvestmentStats}>
                        // todo - edit functionality
                            <p>
                                { format!("name {}", investment.name) }
                            </p>
                            <p>
                                { format!("amount {}", investment.amount) }
                            </p>
                            <p>
                                { format!("earned percentage {}", investment.earned_percentage) }
                            </p>
                        // todo - earned in dollars
                        </Link<Route>>
                    </div>
                }
            }).collect::<Html>()
        }
        </div>
    }
}
