use chrono::{NaiveDate, Utc};
use yew::prelude::*;
use yew::{html, Html};

#[derive(Clone, Properties, PartialEq)]
struct UserBoughtStockHistoryInstance {
    bought_percentage: f64, // sell would be negative
    money_invested: f32,
    date: AttrValue,
}

#[function_component]
pub fn InvestmentStats() -> Html {
    let dt = AttrValue::from(Utc::now().date_naive().to_string());

        let user_bought_stock_history = [
        UserBoughtStockHistoryInstance {
            bought_percentage: 0.00012701754829,
            money_invested: 1328.0,
            date: AttrValue::from(NaiveDate::from_ymd_opt(2021, 4, 2).unwrap().to_string())
        },
        UserBoughtStockHistoryInstance {
            bought_percentage: 0.000472340878,
            money_invested: 3130.0,
            date: AttrValue::from(NaiveDate::from_ymd_opt(2021, 8, 18).unwrap().to_string())
        },
        UserBoughtStockHistoryInstance {
            bought_percentage: 0.00008139847710,
            money_invested: 9344.0,
            date: AttrValue::from(NaiveDate::from_ymd_opt(2022, 2, 20).unwrap().to_string())
        },
    ];

    fn calc_plots(current_stock_value: f32) -> f32 {
        let top_end_stock_value: f32 = 40000.0;
        let bottom_end_stock_value: f32 = 10000.0;
        // calculated by date of bottom and top end stock value
        let stock_value = top_end_stock_value - bottom_end_stock_value;
        let stock_value_in_percentage = current_stock_value / stock_value;
        // CalcPlots { stock_value_in_percentage }
        // stock_value_in_percentage
        stock_value_in_percentage
    }

    // #[derive(Clone, Properties, PartialEq)]
    // struct Props {
    //     #[prop_or_default]
    //     user_bought_stock_history: Vec<UserBoughtStockHistoryInstance>,
    // }

    #[function_component]
    fn Chart(/*props: &Props*/) -> Html {
        let user_bought_stock_history = [
            UserBoughtStockHistoryInstance {
                bought_percentage: 0.00012701754829,
                money_invested: 1328.0,
                date: AttrValue::from(NaiveDate::from_ymd_opt(2021, 4, 2).unwrap().to_string())
            },
            UserBoughtStockHistoryInstance {
                bought_percentage: 0.000472340878,
                money_invested: 3130.0,
                date: AttrValue::from(NaiveDate::from_ymd_opt(2021, 8, 18).unwrap().to_string())
            },
            UserBoughtStockHistoryInstance {
                bought_percentage: 0.00008139847710,
                money_invested: 9344.0,
                date: AttrValue::from(NaiveDate::from_ymd_opt(2022, 2, 20).unwrap().to_string())
            },
        ];
        // let user_bought_stock_history = props.user_bought_stock_history.clone();
        // let pass_user_bought_stock_history = IntoPropValue::into_prop_value(user_bought_stock_history);

        user_bought_stock_history.iter().enumerate().for_each(|(i, x)| {
            println!("Item {} = {}", i, x.money_invested);
        });

        html! {
            <div>
                {
                    user_bought_stock_history.iter().enumerate().map(|(index, item), | {
                        let mut previous_money_invested = 0.0;
                        if index > 0 {
                            previous_money_invested = user_bought_stock_history[index - 1].money_invested;
                        }

                        html!{
                            <svg style="width: 20px;"> <line stroke-width="1px" stroke="#000000" x1="0"
                                x2="20"
                                y1={format!("{}", 20.0 * calc_plots(previous_money_invested))} id="mySVG"
                                y2={format!("{}", 20.0 * calc_plots(item.money_invested))} id="mySVG" />
                            </svg>
                        }
                    }).collect::<Html>()
                }
            </div>
        }
    }

    html! {
        <div>
        // todo - history of bought stocks
            {dt.to_string()}
            <p>{"stocks graph here"}</p>
                // todo - replace with dates
            <Chart />
            <p>
                {"Stock bought history"}
            </p>
            {
                user_bought_stock_history.into_iter().map(|item| {
                    html!{
                        <div class="card" key={format!("{:?}", item.date)}>
                            <p>{format!("Date bought/sold: {:?}", item.date)}</p>
                            <p>{format!("Bought/sold percentage: {:?}", item.bought_percentage)}</p>
                            <p>{format!("Money invested: {:?}", item.money_invested)}</p>
                        </div>
                    }
                }).collect::<Html>()
            }
            <div class="buy-sell">
                <div class="buy">
                    <input placeholder="In dollars" type="text" />
                    <input placeholder="In percentage" type="text" />
                    <button class="button">{"Buy"}</button>
                </div>
                <div class="sell">
                    <input placeholder="In dollars" type="text" />
                    <input placeholder="In percentage" type="text" />
                    <button class="button">{"Sell"}</button>
                </div>
            </div>
        </div>
    }
}
