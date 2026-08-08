# AccordLedger

A generic royalty and commission settlement core based on the reusable ideas in
the JAmerica royalty work and RepCom. It uses integer cents and basis points,
nets returns and chargebacks, honors minimums, and applies recoupable advances.
No organization names, licensing records, rates, or sales data are copied.

```powershell
cargo test
cargo run
```

Next slices: effective-dated agreements and tiers, territory/product splits,
statement imports, exception queues, approval/close, payable export, and a
reproducible settlement statement.
