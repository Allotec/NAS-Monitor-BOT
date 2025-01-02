#!/usr/bin/env node

require("dotenv").config();
const { Client, GatewayIntentBits } = require("discord.js");

const client = new Client({
  intents: [GatewayIntentBits.DirectMessages],
  partials: ["CHANNEL"],
});

const token = process.env.DISCORD_BOT_TOKEN;
const userId = process.env.USER_ID;

client.once("ready", async () => {
  console.log(`Logged in as ${client.user.tag}!`);
  try {
    const user = await client.users.fetch(userId);
    await user.send("Hello! This is a message from your bot.");
    console.log(`Message sent to ${user.tag}`);
  } catch (error) {
    console.error("Failed to send message:", error);
  }

  client.destroy();
});

client.login(token);
