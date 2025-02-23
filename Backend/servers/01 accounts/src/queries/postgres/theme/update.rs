use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::generated::protos::settings::profile::theme::request::{
    request::RequestField, Colour, Colours, Custom, Presets,
};

pub async fn update_theme_from_uuid(
    pool: &Pool<Postgres>,
    theme: RequestField,
    user_uuid: &Uuid,
) -> Result<(), sqlx::Error> {
    match theme {
        RequestField::Presets(theme) => {
            // If theme is a preset then set the preset field to the choice and set is_theme_preset to preset
            let preset = Presets::try_from(theme.to_owned()).unwrap().as_str_name();

            let user_update_query = sqlx::query("UPDATE users SET theme_preset=t.preset, is_theme_preset=TRUE FROM themes AS t WHERE uuid=($1), t.theme = ($2);")
                .bind(user_uuid)
                .bind(preset)
                .execute(pool)
                .await;

            if let Err(err) = user_update_query {
                return Err(err);
            } else {
                return Ok(());
            }
        }
        RequestField::Custom(Custom { primary, secondary }) => {
            let Colours {
                colour_1: primary_1,
                colour_2: primary_2,
                colour_3: primary_3,
                colour_4: primary_4,
                colour_5: primary_5,
                colour_6: primary_6,
            } = primary.unwrap();

            let Colours {
                colour_1: secondary_1,
                colour_2: secondary_2,
                colour_3: secondary_3,
                colour_4: secondary_4,
                colour_5: secondary_5,
                colour_6: secondary_6,
            } = secondary.unwrap();

            let Colour {
                red: red_primary_1,
                green: green_primary_1,
                blue: blue_primary_1,
                alpha: alpha_primary_1,
            } = primary_1.unwrap();
            let Colour {
                red: red_primary_2,
                green: green_primary_2,
                blue: blue_primary_2,
                alpha: alpha_primary_2,
            } = primary_2.unwrap();
            let Colour {
                red: red_primary_3,
                green: green_primary_3,
                blue: blue_primary_3,
                alpha: alpha_primary_3,
            } = primary_3.unwrap();
            let Colour {
                red: red_primary_4,
                green: green_primary_4,
                blue: blue_primary_4,
                alpha: alpha_primary_4,
            } = primary_4.unwrap();
            let Colour {
                red: red_primary_5,
                green: green_primary_5,
                blue: blue_primary_5,
                alpha: alpha_primary_5,
            } = primary_5.unwrap();
            let Colour {
                red: red_primary_6,
                green: green_primary_6,
                blue: blue_primary_6,
                alpha: alpha_primary_6,
            } = primary_6.unwrap();
            let Colour {
                red: red_secondary_1,
                green: green_secondary_1,
                blue: blue_secondary_1,
                alpha: alpha_secondary_1,
            } = secondary_1.unwrap();
            let Colour {
                red: red_secondary_2,
                green: green_secondary_2,
                blue: blue_secondary_2,
                alpha: alpha_secondary_2,
            } = secondary_2.unwrap();
            let Colour {
                red: red_secondary_3,
                green: green_secondary_3,
                blue: blue_secondary_3,
                alpha: alpha_secondary_3,
            } = secondary_3.unwrap();
            let Colour {
                red: red_secondary_4,
                green: green_secondary_4,
                blue: blue_secondary_4,
                alpha: alpha_secondary_4,
            } = secondary_4.unwrap();
            let Colour {
                red: red_secondary_5,
                green: green_secondary_5,
                blue: blue_secondary_5,
                alpha: alpha_secondary_5,
            } = secondary_5.unwrap();
            let Colour {
                red: red_secondary_6,
                green: green_secondary_6,
                blue: blue_secondary_6,
                alpha: alpha_secondary_6,
            } = secondary_6.unwrap();

            let user_update_query = sqlx::query(
                "UPDATE users SET
red_primary_1=($1), green_primary_1=($2), blue_primary_1=($3), alpha_primary_1=($4),
red_primary_2=($5), green_primary_2=($6), blue_primary_2=($7), alpha_primary_2=($8),
red_primary_3=($9), green_primary_3=($10), blue_primary_3=($11), alpha_primary_3=($12),
red_primary_4=($13), green_primary_4=($14), blue_primary_4=($13), alpha_primary_4=($16),
red_primary_5=($17), green_primary_5=($18), blue_primary_5=($19), alpha_primary_5=($20),
red_primary_6=($21), green_primary_6=($22), blue_primary_6=($23), alpha_primary_6=($24),
red_secondary_1=($25), green_secondary_1=($26), blue_secondary_1=($27), alpha_secondary_1=($28),
red_secondary_2=($29), green_secondary_2=($30), blue_secondary_2=($31), alpha_secondary_2=($32),
red_secondary_3=($33), green_secondary_3=($34), blue_secondary_3=($35), alpha_secondary_3=($36),
red_secondary_4=($37), green_secondary_4=($38), blue_primary_4=($39), alpha_secondary_4=($40),
red_secondary_5=($41), green_secondary_5=($42), blue_primary_5=($43), alpha_secondary_5=($44),
red_secondary_6=($45), green_secondary_6=($46), blue_primary_6=($47), alpha_secondary_6=($48),
            WHERE uuid=($39);",
            )
            .bind(red_primary_1 as i32)
            .bind(green_primary_1 as i32)
            .bind(blue_primary_1 as i32)
            .bind(alpha_primary_1 as i32)
            .bind(red_primary_2 as i32)
            .bind(green_primary_2 as i32)
            .bind(blue_primary_2 as i32)
            .bind(alpha_primary_2 as i32)
            .bind(red_primary_3 as i32)
            .bind(green_primary_3 as i32)
            .bind(blue_primary_3 as i32)
            .bind(alpha_primary_3 as i32)
            .bind(red_primary_4 as i32)
            .bind(green_primary_4 as i32)
            .bind(blue_primary_4 as i32)
            .bind(alpha_primary_4 as i32)
            .bind(red_primary_5 as i32)
            .bind(green_primary_5 as i32)
            .bind(blue_primary_5 as i32)
            .bind(alpha_primary_5 as i32)
            .bind(red_primary_6 as i32)
            .bind(green_primary_6 as i32)
            .bind(blue_primary_6 as i32)
            .bind(alpha_primary_6 as i32)
            .bind(red_secondary_1 as i32)
            .bind(green_secondary_1 as i32)
            .bind(blue_secondary_1 as i32)
            .bind(alpha_secondary_1 as i32)
            .bind(red_secondary_2 as i32)
            .bind(green_secondary_2 as i32)
            .bind(blue_secondary_2 as i32)
            .bind(alpha_secondary_2 as i32)
            .bind(red_secondary_3 as i32)
            .bind(green_secondary_3 as i32)
            .bind(blue_secondary_3 as i32)
            .bind(alpha_secondary_3 as i32)
            .bind(red_secondary_4 as i32)
            .bind(green_secondary_4 as i32)
            .bind(blue_secondary_4 as i32)
            .bind(alpha_secondary_4 as i32)
            .bind(red_secondary_5 as i32)
            .bind(green_secondary_5 as i32)
            .bind(blue_secondary_5 as i32)
            .bind(alpha_secondary_5 as i32)
            .bind(red_secondary_6 as i32)
            .bind(green_secondary_6 as i32)
            .bind(blue_secondary_6 as i32)
            .bind(alpha_secondary_6 as i32)
            .bind(user_uuid)
            .execute(pool)
            .await;

            if let Err(err) = user_update_query {
                return Err(err);
            } else {
                return Ok(());
            }
        }
    }
}
