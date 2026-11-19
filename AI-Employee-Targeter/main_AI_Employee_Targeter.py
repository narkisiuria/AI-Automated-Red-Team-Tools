import os
import json
from groq import Groq

class AIEmployeeTargeter:
    def __init__(self, target_company):
        self.target_company = target_company
        self.api_key = os.getenv("GROQ_API_KEY")
        self.client = Groq(api_key=self.api_key)
    
    def call_groq(self, system_prompt: str, user_content: str) -> str:
        """
        Sends a system prompt and user input to the Groq API using a free tier model.
        """
        free_tier_model = "llama-3.1-8b-instant"
        
        try:
            response = self.client.chat.completions.create(
                messages=[
                    {
                        "role": "system",
                        "content": system_prompt
                    },
                    {
                        "role": "user",
                        "content": user_content
                    }
                ],
                model=free_tier_model,
            )

            return response.choices[0].message.content
            
        except Exception as e:
            return f"An error occurred while calling groq: {str(e)}"
    
    def mock_scrape_osint(self):
        """
        Simulates gathering raw text or profiles from public search API/scrapers.
        In production, integrate this with a LinkedIn scraper, Hunter.io, or GitHub API.
        """
        raw_osint_data = [
            {
                "name": "Alex Rivera",
                "title": "Senior DevOps Engineer at " + self.target_company,
                "github_bio": "Working extensively with Kubernetes, Terraform, and Python.",
                "recent_commits": ["Fixing CORS errors in frontend", "Updated AWS IAM roles for staging"],
                "public_posts": "Just migrated our team's internal tools to Slack apps. Super convenient!"
            },
            {
                "name": "Sarah Chen",
                "title": "Junior Frontend Developer at " + self.target_company,
                "github_bio": "React lover. Currently learning Rust.",
                "recent_commits": ["Merged PR #42 in legacy-php-app"],
                "public_posts": "Struggling with Jira tickets today. Anyone else hate sprint planning?"
            }
        ]
        return raw_osint_data

    def analyze_profile_with_ai(self, profile_data):
        """
        Uses an LLM to evaluate the target's high-value technical stack and 
        identify custom social engineering / payload vectors.
        """
        prompt = f"""
        You are an advanced offensive security OSINT analyst engine. 
        Analyze the following employee profile data from our target company ({self.target_company}).
        
        Profile Data: {json.dumps(profile_data)}
        
        Provide a structured breakdown including:
        1. Exploit/Payload Context: What specific tech stack/software do they interact with that we can build a weaponized payload for? (e.g. if they use Kubernetes/Terraform, targets might be malicious Terraform providers).
        2. Phishing Pretext Angle: What specific topic, theme, or authority figure would make a phishing pretext highly convincing to this specific person based on their posts or frustrations?
        
        Return ONLY a clean JSON object matching this structure:
        {{
            "name": "Employee Name",
            "high_value_target": true/false,
            "target_tech_stack": ["tech1", "tech2"],
            "suggested_payload_type": "description of weaponized asset",
            "suggested_pretext_theme": "specific phishing theme"
        }}
        """
        
        # Simulated LLM API Call (adapt to your exact model framework)
        # response = openai.ChatCompletion.create(model="gpt-4o", messages=[{"role": "user", "content": prompt}])
        # return json.loads(response.choices[0].message.content)
        
        # Mocking the AI output matching the schema for the example:

    def run(self):
        print(f"[*] Starting OSINT mapping against: {self.target_company}")
        raw_profiles = self.mock_scrape_osint()
        
        targeted_intel_report = []
        
        for profile in raw_profiles:
            print(f"[*] Processing employee profile for AI analysis: {profile['name']}")
            analysis = self.analyze_profile_with_ai(profile)
            targeted_intel_report.append(analysis)
            
        # Save the structured intelligence output
        output_file = f"{self.target_company.lower().replace(' ', '_')}_intel.json"
        with open(output_file, 'w') as f:
            json.dump(targeted_intel_report, f, indent=4)
            
        print(f"[+] Intelligence gathering complete. Data piped to {output_file}")
        return targeted_intel_report

if __name__ == "__main__":
    # Example Target Company Name
    targeter = AIEmployeeTargeter("TargetCorp Systems")
    targeter.run()
