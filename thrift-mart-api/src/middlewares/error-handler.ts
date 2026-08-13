import type { NextFunction, Request, Response } from "express";
import { isProduction } from "../config/env.js";
import { HttpError } from "../utils/http-error.js";

export function errorHandler(
  err: unknown,
  req: Request,
  res: Response,
  next: NextFunction,
): void {
  const status = err instanceof HttpError ? err.status : 500;
  const message =
    err instanceof Error && (status < 500 || !isProduction)
      ? err.message
      : "Internal server error";

  if (status >= 500) {
    console.error(err);
  }

  res.status(status).json({ error: { message } });
}
