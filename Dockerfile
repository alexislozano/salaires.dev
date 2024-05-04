FROM denoland/deno

EXPOSE 3000

COPY . .

RUN deno cache src/main.ts

CMD ["deno", "task", "run"]
