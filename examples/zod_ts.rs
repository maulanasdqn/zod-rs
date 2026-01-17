use serde::{Deserialize, Serialize};
use zod_rs_ts::ZodTs;

#[derive(Debug, Serialize, Deserialize, ZodTs)]
struct User {
    #[zod(min_length(2), max_length(50))]
    username: String,

    #[zod(email)]
    email: String,

    #[zod(min(18.0), max(120.0), int)]
    age: u32,

    #[zod(min_length(1), max_length(10))]
    interests: Vec<String>,

    bio: Option<String>,

    #[zod(nonnegative)]
    score: f64,

    is_active: bool,
}

#[derive(Debug, Serialize, Deserialize, ZodTs)]
enum Status {
    Active,
    Inactive,
    Pending,
}

#[derive(Debug, Serialize, Deserialize, ZodTs)]
enum Message {
    Text(String),
    Number(i32),
}

#[derive(Debug, Serialize, Deserialize, ZodTs)]
enum Event {
    Click { x: i32, y: i32 },
    Scroll { delta: f64 },
}

fn main() {
    println!("=== User Schema ===\n");
    println!("{}", User::zod_ts());

    println!("\n=== Status Schema ===\n");
    println!("{}", Status::zod_ts());

    println!("\n=== Message Schema ===\n");
    println!("{}", Message::zod_ts());

    println!("\n=== Event Schema ===\n");
    println!("{}", Event::zod_ts());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_zod_ts() {
        let ts = User::zod_ts();
        assert!(ts.contains("import { z } from 'zod'"));
        assert!(ts.contains("export const UserSchema"));
        assert!(ts.contains("username: z.string().min(2).max(50)"));
        assert!(ts.contains("email: z.string().email()"));
        assert!(ts.contains("age: z.number().int().min(18).max(120)"));
        assert!(ts.contains("bio: z.string().optional()"));
        assert!(ts.contains("export type User = z.infer<typeof UserSchema>"));
    }

    #[test]
    fn test_status_zod_ts() {
        let ts = Status::zod_ts();
        assert!(ts.contains("z.union(["));
        assert!(ts.contains("z.object({ Active: z.null() })"));
        assert!(ts.contains("z.object({ Inactive: z.null() })"));
        assert!(ts.contains("z.object({ Pending: z.null() })"));
    }

    #[test]
    fn test_message_zod_ts() {
        let ts = Message::zod_ts();
        assert!(ts.contains("z.object({ Text: z.string() })"));
        assert!(ts.contains("z.object({ Number: z.number().int() })"));
    }

    #[test]
    fn test_event_zod_ts() {
        let ts = Event::zod_ts();
        assert!(ts.contains("Click:"));
        assert!(ts.contains("x: z.number().int()"));
        assert!(ts.contains("Scroll:"));
        assert!(ts.contains("delta: z.number()"));
    }
}
