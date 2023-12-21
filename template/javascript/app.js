require("dotenv").config();

const express = require("express");
const app = express();

const mainRouter = require("./routes/main");

app.use(express.json());

app.use("/", mainRouter);

const port = process.env.PORT || 5000;
app.listen(port, () => {
  console.log(`Server is running on port ${port}`);
});
