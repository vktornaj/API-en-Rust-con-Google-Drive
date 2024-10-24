# Rust API para usar Google Drive

## Configurar credenciales de Google:
- En tu proyecto de google cloud activa la api de google drive.

- Configura la OAuth consent screen con los scopes ncesarios y agrega los correos que deseas usar a testers.

- Crea las credenciales para aplicacion web, establece la url de redireccionamiento en `http://localhost:8080/api/public/callback`.

## Correr el proyecto:

- Ve a la carpeta `.compose` y corre el comando `docker compose up --build -d` el API estara disponible en el puerto `8080`.

## Usar el proyecto:

### Endpoints

- ### GET /api/public/google-auth-url
    Entra a esta pagina desde el navegador y aprueba el uso de google drive por la API.
    Obtendras el `auth_token` (no es el acces token de google). 
    ### Ejemplo de respuesta:
    ```json
    {
        "data":"[auth_token]","error":null
    }
    ```

- ### GET /api/protected/list-files?folder_id=[folder_id]
    Esta es la ruta para listar los archivos en una carpeta especifica.
    ### Ejemplo de la petición:
    ```bash
        curl -X GET "http://localhost:8080/api/protected/list-files?folder_id=[folder_id]" \
            -H "Authorization: Bearer [auth_token]"
    ```
    ### Ejemplo de respuesta:
    ```json
    {
        "data": [
            {
                "id": "[id]",
                "name": "[name]",
                "file_type": "application/pdf",
                "created_at": "2024-10-09T17:44:26.438Z"
            },
            {
                "id": "[id]",
                "name": "[name]",
                "file_type": "application/pdf",
                "created_at": "2024-10-09T17:33:39.018Z"
            },
        ],
        "error": null
    }
    ```

- ### GET /api/protected/download-pdf?file_id=[file_id]
    Esta es la ruta para descargar un archivo pdf en especifico.
    Recibes el archivo.
    ### Ejemplo de la petición:
    ```bash
        curl -X GET http://localhost:8080/api/protected/download-pdf \
            -H "Authorization: Bearer [auth_token]" \
            -d "file_id=[file_id]"
    ```

- ### POST /api/protected/upload-pdf
    Esta es la ruta para subir un archivo pdf.
    ### Ejemplo de la peticion:
    ```bash
        curl -X POST http://localhost:8080/api/protected/upload-pdf \
            -H "Authorization: Bearer [auth_token]" \
            -F "file=@/path/to/your/file.pdf"
    ```
    ### Ejemplo de respuesta:
    ```json
        {
            "kind": "drive#file",
            "id": "[id]",
            "name": "[name]",
            "mimeType": "application/pdf"
        }
    ``` 