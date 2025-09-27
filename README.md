### backendディレクトリの環境変数(.envファイル)
- BASE_URL: apiサーバのurl(http://localhost:8080)
- REDIS_URL: レディスのurl(redis://localhost:6379)
- FRONT_ORIGIN: reactでのオリジン(http://localhost:3000)
- KEY:レディス用の鍵
- GOOGLE_CLIENT_ID:
- GOOGLE_CLIENT_SECRET:
- API_KEY:

### frontendディレクトリの環境変数(env.tsファイル)
- BASE_URL: apiサーバのurl(http://localhost:8080)

### 使用方法
- backend/redisディレクトリでdocker compose up -dを入力してredisサーバを起動する
- backend/actix-appディレクトリでcargo runを入力してapiサーバを起動する
- frontend/react-appディレクトリでnpm startを入力してhttp://localhost:3000を開く
- グーグルのアイコンをクリックし、OAuth認証を行い、そのあとにフォームにyoutubeのプレイリストurlを入力し、送信する。
