use chrono::{DateTime, Months, NaiveDate, NaiveDateTime, Utc};
use instant::Instant;
use yew::prelude::*;
use yew::{html, Html};

struct UserBoughtStockHistoryInstance {
    bought_percentage: f64, // sell would be negative
    money_invested: i32,
    date: AttrValue,
}

#[function_component]
pub fn InvestmentStats() -> Html {
    let dt = AttrValue::from(Utc::now().date_naive().to_string());

    let user_bought_stock_history = [
        UserBoughtStockHistoryInstance {
            bought_percentage: 0.00012701754829,
            money_invested: 1328,
            date: AttrValue::from(NaiveDate::from_ymd_opt(2021, 4, 2).unwrap().to_string())
        },
        UserBoughtStockHistoryInstance {
            bought_percentage: 0.000472340878,
            money_invested: 3130,
            date: AttrValue::from(NaiveDate::from_ymd_opt(2021, 8, 18).unwrap().to_string())
        },
        UserBoughtStockHistoryInstance {
            bought_percentage: 0.00008139847710,
            money_invested: 9344,
            date: AttrValue::from(NaiveDate::from_ymd_opt(2022, 2, 20).unwrap().to_string())
        },
    ];

    html! {
        <div>
        // todo - history of bought stocks
            {dt.to_string()}
            <p>{"stocks graph here"}</p>
                // todo - replace with dates
                <dd hover="df" class="percentage percentage-11"><span class="text">{"IE 11: 11.33%"}</span></dd>
                <dd class="percentage percentage-49"><span class="text">{"Chrome: 49.77%"}</span></dd>
                <dd class="percentage percentage-16"><span class="text">{"Firefox: 16.09%"}</span></dd>
                <dd class="percentage percentage-5"><span class="text">{"Safari: 5.41%"}</span></dd>
                <dd class="percentage percentage-2"><span class="text">{"Opera: 1.62%"}</span></dd>
                <dd class="percentage percentage-2"><span class="text">{"Android 4.4: 2%"}</span></dd>
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
