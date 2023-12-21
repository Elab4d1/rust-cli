import express from "express";
const router = express.Router();

// Import controllers
import { getHello } from "../controllers/main";

// Define routes
router.get("/", getHello);

export default router;
