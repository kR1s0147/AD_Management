from fastapi import FastAPI, HTTPException
from fastapi.middleware.cors import CORSMiddleware
from .models import SearchURLsInput, CategorizationResponse, ErrorResponse
from .categorizer import AdCategorizingAgent

app = FastAPI(
    title="Ad Categorization API",
    description="API for analyzing search URLs and categorizing user interests",
    version="1.0.0"
)

# Add CORS middleware
app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

# Initialize the categorization agent
categorizer = AdCategorizingAgent()

@app.post(
    "/analyze", 
    response_model=CategorizationResponse,
    responses={400: {"model": ErrorResponse}},
    tags=["Categorization"]
)
async def analyze_searches(input_data: SearchURLsInput):
    """
    Analyze a list of search URLs to determine user interests and ad categories
    """
    try:
        result = categorizer.analyze_searches(input_data)
        
        if "error" in result:
            raise HTTPException(
                status_code=400,
                detail=result
            )
            
        return result
    except Exception as e:
        raise HTTPException(
            status_code=400,
            detail={"error": str(e), "raw_input": input_data}
        )