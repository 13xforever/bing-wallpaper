using System;
using System.Text;

namespace bing_wallpaper_net
{
    internal static class Utils
    {
        public static string ToHexString(this byte[] bytes)
        {
            if (bytes == null || bytes.Length == 0)
                return "";

            var result = new StringBuilder(bytes.Length*2);
            foreach (var b in bytes)
                result.Append(b.ToString("x2"));
            return result.ToString();
        }
    }
}