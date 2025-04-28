from typing import Dict, List
from pydantic import BaseModel, Field

class SearchURLsInput(BaseModel):
    urls: List[str] = Field(
        ..., 
        description="List of search URLs to analyze",
        min_items=1
    )

class CategorizationResponse(BaseModel):
    main_topics: List[str]
    primary_ad_category: str
    secondary_ad_categories: List[str]
    interest_strength: Dict[str, int]

class ErrorResponse(BaseModel):
    error: str
    raw_input: str