# Build and run

With Docker:
```bash
docker build . -t rss-creator
docker run -e "AUTH_USERNAME=" -e "AUTH_PASSWORD=" -p 8080:8080 -d --name simple_rss_creator rss-creator 
```