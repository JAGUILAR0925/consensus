from hashlib import sha256
from fastapi import FastAPI
from pydantic import BaseModel, Field

app = FastAPI(title="Consensus biometric lab", version="0.1.0")
class SyntheticEmbedding(BaseModel):
    values: list[float] = Field(min_length=4, max_length=512)
class Eligibility(BaseModel):
    eligible: bool
    opaque_id: str

@app.get("/health")
def health(): return {"status": "ok", "mode": "synthetic-only"}

@app.post("/v1/eligibility", response_model=Eligibility)
def eligibility(payload: SyntheticEmbedding):
    """Demostración determinista; no acepta imágenes ni se conecta a Stellar."""
    digest = sha256(",".join(f"{v:.6f}" for v in payload.values).encode()).hexdigest()
    return Eligibility(eligible=True, opaque_id=digest[:32])
