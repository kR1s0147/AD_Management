from pydantic_settings import BaseSettings

class Settings(BaseSettings):
    google_api_key: str = ""
    model_name: str = "gemini-1.5-flash"
    temperature: float = 0.3

    class Config:
        env_file = ".env"

settings = Settings()
