use clap::Parser;

/// HTTP client with automatic x402 payment handling
///
/// Prints JavaScript code using x402-client to make payments
#[derive(Parser, Debug)]
#[command(name = "x402curl")]
#[command(version, about, long_about = None)]
struct Args {
    /// The payment URL to access
    payment_url: String,

    /// Maximum auto-payment amount in human units (e.g., 5 for 5 USDC)
    #[arg(long)]
    max_payment: Option<f64>,
}

fn main() {
    let args = Args::parse();

    let code = generate_code(&args.payment_url, args.max_payment);
    println!("{}", code);
}

fn generate_code(payment_url: &str, max_payment: Option<f64>) -> String {
    let max_payment_line = if let Some(max) = max_payment {
        format!("\n  maxPayment: {},", max)
    } else {
        String::new()
    };

    format!(
        r#"import {{ createX402Client }} from "x402-client";
import {{ readFileSync }} from "fs";
import {{ homedir }} from "os";
import {{ join }} from "path";

// Load wallet from ~/.payment/wallet.json
const walletPath = join(homedir(), ".payment", "wallet.json");
const passwordPath = join(homedir(), ".payment", "password.txt");

const keystore = JSON.parse(readFileSync(walletPath, "utf-8"));
const password = readFileSync(passwordPath, "utf-8").trim();

const TARGET_URL = "{}";

const client = await createX402Client({{
  chain: "base",
  keystore: keystore,
  password: password,{}
}});

const response = await client.fetchWithPayment(TARGET_URL, {{ method: "GET" }});
console.log("Response headers:", Object.fromEntries(response.headers));
console.log("Response body:", await response.text());

const paymentHeader = response.headers.get("x-payment-response");
if (paymentHeader) {{
  console.log("Decoded payment response:", client.decodePaymentResponse(paymentHeader));
}}"#,
        payment_url, max_payment_line
    )
}
