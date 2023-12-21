import express from "express";
import mainRouter from "./routes/main";
import dotenv from "dotenv";

dotenv.config();

const app = express();

app.use(express.json());

// Use routers
app.use("/", mainRouter);

const port = process.env.PORT || 3000;
app.listen(port, () => {
  console.log(`Server is running on port ${port}`);
});
