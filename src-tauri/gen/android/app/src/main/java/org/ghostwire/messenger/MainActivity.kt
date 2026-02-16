import android.Manifest
import android.content.Intent
import android.content.pm.PackageManager
import android.os.Build
import android.os.Bundle
import androidx.activity.enableEdgeToEdge
import androidx.core.app.ActivityCompat
import androidx.core.content.ContextCompat

class MainActivity : TauriActivity() {
  override fun onCreate(savedInstanceState: Bundle?) {
    enableEdgeToEdge()
    super.onCreate(savedInstanceState)
    
    checkPermissions()
    startMeshService()
  }

  private fun checkPermissions() {
    if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.TIRAMISU) {
      if (ContextCompat.checkSelfPermission(this, Manifest.permission.POST_NOTIFICATIONS) != PackageManager.PERMISSION_GRANTED) {
        ActivityCompat.requestPermissions(this, arrayOf(Manifest.permission.POST_NOTIFICATIONS), 101)
      }
    }
    
    // Request ignoring battery optimizations for persistent mesh
    if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.M) {
      val intent = Intent()
      val packageName = packageName
      val pm = getSystemService(POWER_SERVICE) as android.os.PowerManager
      if (!pm.isIgnoringBatteryOptimizations(packageName)) {
        intent.action = android.provider.Settings.ACTION_REQUEST_IGNORE_BATTERY_OPTIMIZATIONS
        intent.data = android.net.Uri.parse("package:$packageName")
        startActivity(intent)
      }
    }
  }

  private fun startMeshService() {
    val serviceIntent = Intent(this, MeshService::class.java)
    if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
      startForegroundService(serviceIntent)
    } else {
      startService(serviceIntent)
    }
  }
}
