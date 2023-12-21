import { Request, Response } from "express";

export const getHello = (req: Request, res: Response) => {
  res.send("Hello, World!");
};
