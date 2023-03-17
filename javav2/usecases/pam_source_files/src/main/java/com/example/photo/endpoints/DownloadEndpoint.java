package com.example.photo.endpoints;

import com.example.photo.PhotoApplicationResources;
import com.example.photo.services.DynamoDBService;
import com.example.photo.services.S3Service;
import com.example.photo.services.SnsService;
import java.io.IOException;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.Set;
import java.util.stream.Collectors;
import java.util.stream.Stream;

public class DownloadEndpoint {
    final DynamoDBService dbService;
    final S3Service s3Service;
    final SnsService snsService;

    public DownloadEndpoint(DynamoDBService dynamoDBService, S3Service s3Service, SnsService snsService) {
        this.dbService = dynamoDBService;
        this.s3Service = s3Service;
        this.snsService = snsService;
    }

    public String download(List<String> labels) {
        try {
            // Now we have an image list, place them into a ZIP and presign it.
            Set<String> images = labels.stream().parallel().flatMap(
                (label) -> this.dbService.getImagesByLabel(label).stream())
                .collect(Collectors.toSet());
            Map<String, byte[]> imageMap = new HashMap<>();

            for (String imageName : images) {
                byte[] imageBytes = this.s3Service.getObjectBytes(PhotoApplicationResources.STORAGE_BUCKET, imageName);
                imageMap.put(imageName, imageBytes);
                System.out.println("Add " + imageName + " to the map.");
            }

            // Now we need to ZIP the images.
            byte[] zipFile = this.s3Service.listBytesToZip(imageMap);
            String uuid = java.util.UUID.randomUUID().toString();
            String zipName = uuid + ".zip";

            // Place the zip file into the working bucket and get back a presigned URL.
            s3Service.putObject(zipFile, PhotoApplicationResources.WORKING_BUCKET, zipName);
            String presignedURL = s3Service.signObjectToDownload(PhotoApplicationResources.WORKING_BUCKET, zipName);
            String message = "Your Archived images can be located here " + presignedURL;
            this.snsService.pubTopic(message);
            return presignedURL;
        } catch (IOException e) {
            e.getMessage();
        }
        return null;
    }
}
