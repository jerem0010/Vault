import express, { Request, Response } from "express";
import bodyParser from "body-parser";
import { deposit, getBalance } from "./solana";

const app = express();
app.use(bodyParser.json());

app.post("/deposit", async (req: Request, res: Response) => {
  const { wallet, amount } = req.body;
  const tx = await deposit(wallet, amount);
  res.json({ success: true, tx });
});

app.get("/balance/:wallet", async (req: Request, res: Response) => {
  const { wallet } = req.params;
  const balance = await getBalance(wallet);
  res.json({ wallet, balance });
});

app.listen(3000, () => {
  console.log("🚀 Backend running on http://localhost:3000");
});
