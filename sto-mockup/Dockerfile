# Biyard STO Mockup — 정적 HTML 동결 이미지
#
# 빌드:   docker build -t biyard-sto-mockup:latest .
# 실행:   docker run -d --name biyard-sto-mockup -p 8082:80 biyard-sto-mockup:latest

FROM python:3.12-alpine

WORKDIR /app

COPY index.html assets.html detail.html news.html pricing.html \
     biyard-index.html index-whitepaper.html launchpad.html \
     app.js data.js data-dart-backfill.js data-artipio.js data-musicow.js news.js \
     styles.css \
     biyard-logo.png \
     mock-db.json \
     /app/

EXPOSE 80

CMD ["python", "-m", "http.server", "80", "--bind", "0.0.0.0"]
