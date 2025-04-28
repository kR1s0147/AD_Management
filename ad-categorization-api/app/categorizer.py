from typing import List
import google.generativeai as genai
from langchain_google_genai import GoogleGenerativeAI
from langchain.prompts import PromptTemplate
from langchain_core.output_parsers import JsonOutputParser
from langchain_core.runnables import RunnablePassthrough

from .config import settings

class AdCategorizingAgent:
    def __init__(self):
        # Initialize Gemini LLM
        genai.configure(api_key=settings.google_api_key)
        self.llm = GoogleGenerativeAI(
            model=settings.model_name,
            google_api_key=settings.google_api_key,
            temperature=settings.temperature
        )
        
        # Define the URL analysis prompt with proper JSON escaping
        self.prompt = PromptTemplate(
            template="""
            Analyze these search URLs and understand the user's interests and behavior patterns.
            Consider both explicit topics and implicit patterns in their search behavior.
            
            Search URLs:
            {search_urls}
            
            Provide a detailed analysis in JSON format including the main topics discovered, primary advertising category, 
            secondary advertising categories, and interest strength for each category (scale 1-10).
            
            Return your response in this exact JSON format:
            {{
                "main_topics": [],
                "primary_ad_category": "",
                "secondary_ad_categories": [],
                "interest_strength": {{}}
            }}
            """,
            input_variables=["search_urls"]
        )

        # Setup the processing chain
        self.parser = JsonOutputParser()
        self.chain = (
            {"search_urls": RunnablePassthrough()} 
            | self.prompt 
            | self.llm 
            | self.parser
        )

    def analyze_searches(self, search_urls: List[str]) -> dict:
        """Analyze a list of search URLs and return categorization"""
        try:
            # Extract search queries from URLs and handle both regular URLs and search URLs
            search_queries = []
            for url in search_urls:
                if "google.com/search?q=" in url:
                    query = url.split("q=")[-1].replace("+", " ")
                else:
                    # For non-Google search URLs, extract the last part of the path
                    query = url.split("/")[-1].replace("-", " ")
                search_queries.append(query)
            
            formatted_searches = "\n".join(search_queries)
            
            result = self.chain.invoke(formatted_searches)
            return result
        except Exception as e:
            return {
                "error": f"Analysis failed: {str(e)}",
                "raw_input": formatted_searches
            }