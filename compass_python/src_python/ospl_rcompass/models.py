from datetime import date

from pydantic import BaseModel
from pydantic import Field
from pydantic import field_validator
from pydantic import model_validator


class Shot(BaseModel):
    from_: str = Field(..., alias="from", min_length=1, max_length=32)
    to: str = Field(..., min_length=1, max_length=32)
    length: float = Field(..., gt=0, description="Shot length in unit system")
    azimuth: float = Field(..., ge=0, lt=360, description="Bearing in degrees [0–360)")
    depth: float = Field(..., description="Depth change (positive = down)")
    # LRUD
    left: float | str | None = Field(None)
    right: float | str | None = Field(None)
    up: float | str | None = Field(None)
    down: float | str | None = Field(None)

    flags: str | None = Field(None, max_length=64)
    comment: str | None = Field(None, max_length=256)

    # Convert stringified numbers to floats
    @field_validator(
        "length", "azimuth", "depth", "left", "right", "up", "down", mode="before"
    )
    @classmethod
    def parse_numeric_str(cls, v):
        if v in ("", None):
            return None
        try:
            return float(v)
        except (TypeError, ValueError) as e:
            raise ValueError(f"Expected numeric or empty value, got {v!r}") from e

    @model_validator(mode="after")
    def check_measurements(self):
        # Ensure Left/Right/Up/Down are non-negative if present
        for field_name in ("left", "right", "up", "down"):
            value = getattr(self, field_name)
            if value is not None and value < 0:
                raise ValueError(f"{field_name} cannot be negative")

        return self


class SurveyData(BaseModel):
    shots: list[Shot] = Field(..., min_length=1)
    survey_date: date
    unit: str = Field(..., pattern=r"^(feet|meters)$", description="Supported units")
    cave_name: str = Field(..., min_length=1, max_length=64)
    survey_name: str = Field(..., min_length=1, max_length=64)
    survey_team: list[str] = Field(..., min_length=1)
    comment: str | None = Field(None, max_length=512)
    latitude: float = Field(..., ge=-90.0, le=90.0)
    longitude: float = Field(..., ge=-180.0, le=180.0)
    declination: float = Field(..., ge=-90.0, le=90.0)

    @field_validator("survey_team")
    @classmethod
    def strip_team_names(cls, v: list[str]) -> list[str]:
        return [name.strip() for name in v if name.strip()]
