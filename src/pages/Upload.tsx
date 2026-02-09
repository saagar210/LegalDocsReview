import { useState, useCallback } from "react";
import { useNavigate } from "react-router";
import { useDropzone } from "react-dropzone";
import { open } from "@tauri-apps/plugin-dialog";
import { Upload as UploadIcon, FileText, X } from "lucide-react";
import toast from "react-hot-toast";
import { uploadDocument, extractDocumentText } from "@/lib/commands";
import { CONTRACT_TYPE_LABELS } from "@/types";
import type { ContractType } from "@/types";

function Upload() {
  const navigate = useNavigate();
  const [selectedFile, setSelectedFile] = useState<string | null>(null);
  const [fileName, setFileName] = useState<string | null>(null);
  const [contractType, setContractType] = useState<ContractType>("nda");
  const [uploading, setUploading] = useState(false);

  const handleSelectFile = useCallback(async () => {
    const result = await open({
      filters: [{ name: "PDF", extensions: ["pdf"] }],
      multiple: false,
    });
    if (result) {
      setSelectedFile(result);
      const parts = result.split(/[/\\]/);
      setFileName(parts[parts.length - 1] ?? result);
    }
  }, []);

  const onDrop = useCallback((_acceptedFiles: File[]) => {
    // In Tauri, drag-drop gives us file paths via the native dialog
    // For now, prompt the user to use the file picker
    handleSelectFile();
  }, [handleSelectFile]);

  const { getRootProps, getInputProps, isDragActive } = useDropzone({
    onDrop,
    noClick: true,
    accept: { "application/pdf": [".pdf"] },
  });

  const handleUpload = useCallback(async () => {
    if (!selectedFile) return;

    setUploading(true);
    try {
      const doc = await uploadDocument(selectedFile, contractType);
      toast.success("Document uploaded successfully");

      // Start text extraction immediately
      try {
        await extractDocumentText(doc.id);
        toast.success("Text extracted from PDF");
      } catch (err) {
        toast.error(
          `Text extraction failed: ${err instanceof Error ? err.message : String(err)}`,
        );
      }

      navigate(`/documents/${doc.id}`);
    } catch (err) {
      toast.error(
        `Upload failed: ${err instanceof Error ? err.message : String(err)}`,
      );
    } finally {
      setUploading(false);
    }
  }, [selectedFile, contractType, navigate]);

  return (
    <div className="p-8 max-w-2xl mx-auto">
      <h1 className="text-2xl font-bold mb-2">Upload Document</h1>
      <p className="text-gray-500 mb-8">
        Upload a PDF contract for AI-powered review and analysis
      </p>

      <div
        {...getRootProps()}
        onClick={handleSelectFile}
        className={`border-2 border-dashed rounded-xl p-12 text-center cursor-pointer transition-colors ${
          isDragActive
            ? "border-brand-400 bg-brand-50"
            : selectedFile
              ? "border-green-300 bg-green-50"
              : "border-gray-300 hover:border-brand-400 hover:bg-gray-50"
        }`}
      >
        <input {...getInputProps()} />
        {selectedFile ? (
          <div className="flex flex-col items-center gap-3">
            <FileText className="h-12 w-12 text-green-500" />
            <div>
              <p className="font-medium text-gray-900">{fileName}</p>
              <button
                onClick={(e) => {
                  e.stopPropagation();
                  setSelectedFile(null);
                  setFileName(null);
                }}
                className="text-sm text-red-500 hover:text-red-700 mt-1 flex items-center gap-1 mx-auto"
              >
                <X className="h-3 w-3" /> Remove
              </button>
            </div>
          </div>
        ) : (
          <div className="flex flex-col items-center gap-3">
            <UploadIcon className="h-12 w-12 text-gray-400" />
            <div>
              <p className="font-medium text-gray-900">
                Click to select a PDF
              </p>
              <p className="text-sm text-gray-500 mt-1">
                Or drag and drop a file here
              </p>
            </div>
          </div>
        )}
      </div>

      <div className="mt-6">
        <label className="block text-sm font-medium text-gray-700 mb-2">
          Contract Type
        </label>
        <div className="grid grid-cols-3 gap-3">
          {(Object.entries(CONTRACT_TYPE_LABELS) as [ContractType, string][]).map(
            ([value, label]) => (
              <button
                key={value}
                onClick={() => setContractType(value)}
                className={`p-3 rounded-lg border text-sm font-medium transition-colors ${
                  contractType === value
                    ? "border-brand-500 bg-brand-50 text-brand-700"
                    : "border-gray-200 text-gray-600 hover:border-gray-300"
                }`}
              >
                {label}
              </button>
            ),
          )}
        </div>
      </div>

      <button
        onClick={handleUpload}
        disabled={!selectedFile || uploading}
        className="mt-8 w-full bg-brand-600 text-white py-3 rounded-lg font-medium hover:bg-brand-700 disabled:opacity-50 disabled:cursor-not-allowed transition-colors flex items-center justify-center gap-2"
      >
        {uploading ? (
          <>
            <div className="animate-spin h-4 w-4 border-2 border-white border-t-transparent rounded-full" />
            Processing...
          </>
        ) : (
          <>
            <UploadIcon className="h-4 w-4" />
            Upload & Extract Text
          </>
        )}
      </button>
    </div>
  );
}

export default Upload;
