using System;
using System.IO;
using System.Security.Cryptography;
using System.Net;
using System.Json;
using System.Linq;

namespace bing_wallpaper_net
{
    internal static class Program
    {
        private static readonly string[] markets = {"ja-JP", "en-US", "en-UK", "en-AU", "en-NZ", "en-CA", "de-DE", "zh-CN"};
        private const string requestBaseUrl = "https://www.bing.com/HPImageArchive.aspx?format=js&idx=0&n=1&mkt=";
        private const string destFilename = "/usr/share/antergos/wallpapers/bing_wallpaper.jpg";

        internal static void Main(string[] args)
        {
#if DEBUG
            var absolutePath = Path.GetFullPath(destFilename);
            Console.WriteLine("Full dest path: " + absolutePath);
#endif
            byte[] existingHash = null;
            if (File.Exists(destFilename))
            {
                using (var f = File.Open(destFilename, FileMode.Open, FileAccess.Read, FileShare.Read))
                using (var h = SHA256.Create())
                    existingHash = h.ComputeHash(f);
#if DEBUG
                Console.WriteLine("Current wallpaper hash: " + existingHash.ToHexString());
#endif
            }
            else
            {
                var destDir = Path.GetDirectoryName(destFilename);
                if (!Directory.Exists(destDir))
                {
#if DEBUG
                    Console.WriteLine("Creating new directory " + destDir);
#endif
                    Directory.CreateDirectory(destDir);
                }
            }
            var wallpaperUri = GetWallpaperLink(markets[0]);
#if DEBUG
            Console.WriteLine("Wallpaper link: " + wallpaperUri);
#endif
            var wallpaperBytes = GetWallpaper(wallpaperUri);
            byte[] hash = null;
            using (var h = SHA256.Create())
                hash = h.ComputeHash(wallpaperBytes);
#if DEBUG
            Console.WriteLine("New wallpaper hash: " + hash.ToHexString());
#endif
            if (existingHash != null && hash.SequenceEqual(existingHash))
            {
#if DEBUG
                Console.WriteLine("Hashes are equal, nothing to do");
#endif
                return;
            }

            using (var ms = new MemoryStream(wallpaperBytes))
            using (var f = File.Open(destFilename, FileMode.Create, FileAccess.Write, FileShare.None))
                ms.CopyTo(f);
        }

        private static Uri GetWallpaperLink(string market)
        {
            var uri = new Uri(requestBaseUrl + market);
            var request = (HttpWebRequest)WebRequest.Create(uri);
            request.Accept = "application/json";
            using (var ms = new MemoryStream())
            {
                using (var response = (HttpWebResponse)request.GetResponse())
                using (var s = response.GetResponseStream())
                {
                    s.CopyTo(ms);
                    response.Close();
                }
                ms.Seek(0, SeekOrigin.Begin);
                var json = JsonValue.Load(ms);
                var uriBase = new Uri(uri.GetComponents(UriComponents.SchemeAndServer, UriFormat.SafeUnescaped));
                var relativePath = (string)json["images"][0]["url"];
                return new Uri(uriBase, relativePath);
            }
        }

        private static byte[] GetWallpaper(Uri wallpaperUri)
        {
            var request = (HttpWebRequest)WebRequest.Create(wallpaperUri);
            request.Accept = "application/octet-stream";
            using (var response = (HttpWebResponse)request.GetResponse())
            using (var s = response.GetResponseStream())
            using (var ms = new MemoryStream())
            {
                s.CopyTo(ms);
                return ms.ToArray();
            }
        }
    }
}
