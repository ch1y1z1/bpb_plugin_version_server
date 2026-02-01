# API 文档

## 概述

版本号查询服务，提供键值对的存储和查询功能。

**Base URL:** `http://localhost:3000`

---

## API 列表

### 1. 获取版本号

根据键获取对应的版本号值。

- **请求**
  - **方法:** `GET`
  - **路径:** `/get/{key}`
  - **路径参数:**
    - `key` (string, 必需): 版本号键，如 `"1.0.0"`

- **响应**
  - **状态码:** `200 OK`
  - **Content-Type:** `application/json`
  - **响应体:**
    ```json
    {
      "value": "1.0.10b"
    }
    ```
  
- **错误情况**
  - 键不存在时返回：`{"value": null}`

- **示例**
  ```bash
  curl http://localhost:3000/get/1.0.0
  ```
  
  成功响应：
  ```json
  {
    "value": "1.0.10b"
  }
  ```

---

### 2. 设置版本号

存储或更新键值对。

- **请求**
  - **方法:** `POST`
  - **路径:** `/set`
  - **Content-Type:** `application/json`
  - **请求体:**
    ```json
    {
      "key": "1.0.0",
      "value": "1.0.10b"
    }
    ```
  - **字段说明:**
    - `key` (string, 必需): 版本号键
    - `value` (string, 必需): 版本号值

- **响应**
  - **状态码:** `200 OK`
  - **Content-Type:** `application/json`
  - **响应体:**
    ```json
    {
      "success": true
    }
    ```

- **示例**
  ```bash
  curl -X POST http://localhost:3000/set \
    -H "Content-Type: application/json" \
    -d '{"key":"1.0.0","value":"1.0.10b"}'
  ```
  
  成功响应：
  ```json
  {
    "success": true
  }
  ```

---

## 数据说明

- 键和值均为版本号格式的字符串，如 `"1.0.0"`、`"1.0.10b"`
- 数据持久化存储，服务重启后数据不丢失
- 设置已存在的键会更新其值
