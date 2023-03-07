package com.example.photo.handlers;

import com.amazonaws.services.lambda.runtime.Context;
import com.amazonaws.services.lambda.runtime.RequestHandler;
import com.example.photo.services.S3Service;
import com.google.gson.Gson;

import java.util.Map;
import java.util.UUID;

public class UploadHandler implements RequestHandler<Map<String, String>, String> {

    @Override
    public String handleRequest(Map<String, String> event, Context context) {
        String body = event.get("body");
        context.getLogger().log("Got body: " + body);
        String fileName = event.get("file_name");
        context.getLogger().log("Building URL for " + fileName);
        UUID uuid = UUID.randomUUID();
        String uniqueFileName = uuid + "-" + fileName;

        S3Service s3Service = new S3Service();

        String signedURL = s3Service.signObjectToUpload(uniqueFileName);

        UploadResponse data = UploadResponse.from(signedURL);

        Gson gson = new Gson();
        return gson.toJson(data);
    }

    static class UploadResponse {
        private final String url;

        static UploadResponse from(String url) {
            return new UploadResponse(url);
        }

        private UploadResponse(String url) {
            this.url = url;
        }

        public String getURL() {
            return url;
        }
    }
}